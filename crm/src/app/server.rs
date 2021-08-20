use std::net::SocketAddr;

use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::sync::oneshot::Receiver;
use tracing::{info, warn};

use crate::app::context::RefContext;
use crate::app::panic::PanicReceiver;

pub async fn run(
    context: RefContext,
    shutdown_signal: Receiver<()>,
    mut panic_receiver: PanicReceiver,
) {
    let routes = crate::routes::create(context);
    let address = context.config.http_server.to_server_address();
    let (address, server) = warp::serve(routes).bind_with_graceful_shutdown(address, async move {
        shutdown_signal
            .await
            .expect("Fail received shutdown signal");
        info!("Server shutdown by signal");
    });
    // This is additional server that expect to serve management request only.
    let mut management_server = ManagementServer::create(context);

    display_socket_address("Server", address);
    if let Some(socket_address) = management_server.address() {
        display_socket_address("Management server", socket_address);
    }
    // Run until one of following conditions are met.
    tokio::select! {
        biased;
        // Received panic from async task.
        //
        // We expect `panic_receiver` has never been closed.
        // Closing it will abort process when next panic is sent from sender side.
        // See `crate::app::runtime::spawn_task_handle_panic`.
        Some(panic) = panic_receiver.recv() => {
            std::panic::resume_unwind(panic);
        },
        // Server exit, probably because shutdown signal has been received.
        () = server => {
            warn!("Main server shutdown");
        },
        // Management server exit. If there is no management server, this branch will be disabled.
        Some(()) = management_server.handler() => {
            warn!("Management server shutdown");
        },
    }
}

fn display_socket_address(name: &str, socket_address: SocketAddr) {
    let socket_info = format!("{} is listening on http://{}", name, socket_address);
    eprintln!(">> {}", socket_info);
    info!("{}", socket_info);
}

struct ManagementServer(Option<ManagementServerInner>);

struct ManagementServerInner {
    address: SocketAddr,
    handler: BoxFuture<'static, ()>,
}

impl ManagementServer {
    fn create(context: RefContext) -> Self {
        let data = context
            .config
            .http_server
            .to_management_server_address()
            .map(|address| {
                let routes = crate::routes::create_management(context);
                let handler = warp::serve(routes).bind(address).boxed();
                ManagementServerInner { address, handler }
            });
        Self(data)
    }

    fn address(&self) -> Option<SocketAddr> {
        Some(self.0.as_ref()?.address)
    }

    /// Run management server until it exit
    ///
    /// None is returned if there is no management server
    async fn handler(&mut self) -> Option<()> {
        let unit: () = self.0.take()?.handler.await;
        Some(unit)
    }
}
