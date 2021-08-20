use warp::reply::Response;
use warp::Rejection;

pub async fn panic() -> Result<Response, Rejection> {
    foo().await;
    Ok(Response::default())
}

async fn foo() {
    bar().await;
}

async fn bar() {
    baz().await;
}

async fn baz() {
    panic!("panic in baz");
}
