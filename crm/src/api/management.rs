use warp::reply::Response;

use crate::app::context::RefContext;

pub fn print_config(context: RefContext) -> Response {
    eprintln!("Current configuration");
    eprintln!("{}", context.config.to_pretty_toml());
    Response::default()
}
