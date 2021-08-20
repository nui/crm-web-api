use warp::{path, Filter, Rejection};

use filters::{auth_get, auth_json_post, json_post};
use request_info::make_request_logger;

use crate::api;
use crate::app::context::RefContext;
use crate::auth::filters::extract_access_token;
use crate::auth::rbac::Policy;

mod filters;
mod request_info;

macro_rules! complete_route {
    () => (impl Filter<Extract = (impl warp::Reply,), Error = std::convert::Infallible> + Clone);
}

macro_rules! partial_route {
    () => (impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone);
}

/// Create routing
///
/// Make sure you start filter with path before specify warp::get() or warp::post()
///   GOOD `let foo = path!("bar").and(path::get())`
///   BAD  `let foo = path::get().and(path!("bar"))`
pub fn create(context: RefContext) -> complete_route!() {
    let auth = path!("auth" / ..).and(make_auth_route(context));

    let account = path!("account" / ..).and(combine!(
        // account created by this route can't login until `allowed_login` flag is set
        path!("create")
            .and(auth_json_post(Policy::CreateAccount, context))
            .and_then(api::auth::account::create),
        path!("list")
            .and(auth_get(Policy::ListAccounts, context))
            .and_then(api::auth::account::list),
    ));

    let dev = path!("dev" / ..).and(combine!(
        path!("parse-token")
            .and(auth_json_post(Policy::ParseAccessToken, context))
            .and_then(api::auth::token::parse),
        path!("encode-password")
            .and(auth_json_post(Policy::EncodePassword, context))
            .and_then(api::auth::password::encode),
    ));

    let build_info = path::end().and_then(api::build::info);

    let system_stats = path!("stats")
        .and(warp::get())
        .and(context.extract())
        .and_then(api::stats::system_stats);

    let request_info = path!("request-headers")
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .map(api::echo_header::echo_header);

    let panic = path!("panic").and_then(api::panic::panic);

    let routes = combine!(
        build_info,
        request_info,
        auth,
        account,
        dev,
        panic,
        system_stats
    );
    routes
        .recover(crate::app::rejection::handle_rejection)
        .with(warp::log::custom(make_request_logger(&context.config)))
}

fn make_auth_route(context: RefContext) -> partial_route!() {
    combine!(
        path!("sign-in")
            .and(json_post(context))
            .and_then(api::auth::sign_in),
        path!("refresh-token")
            .and(warp::get())
            .and(extract_access_token(context))
            .and(context.extract())
            .and_then(api::auth::token::refresh),
        path!("change-password")
            .and(json_post(context))
            .and_then(api::auth::password::change),
    )
}

/// This routes is meant to be called from localhost on management port
pub fn create_management(context: RefContext) -> complete_route!() {
    let jemalloc = path!("jemalloc" / ..).and({
        path!("background-thread" / ..).and(combine!(
            path!("enable")
                .and(warp::get())
                .and_then(api::jemalloc::enable_background_thread),
            path!("disable")
                .and(warp::get())
                .and_then(api::jemalloc::disable_background_thread),
        ))
    });

    let big_alloc = path!("big-alloc")
        .and(warp::get())
        .and_then(api::big_alloc::big_alloc);

    let print_config = path!("print-config")
        .and(warp::get())
        .and(context.extract())
        .map(api::management::print_config);

    combine!(jemalloc, big_alloc, print_config)
        .recover(crate::app::rejection::handle_rejection)
        .with(warp::log::custom(make_request_logger(&context.config)))
}
