use serde::de::DeserializeOwned;
use warp::{Filter, Rejection};

use tokidator::rbac::PolicyCondition;

use crate::app::context::RefContext;
use crate::auth::filters::check_permission;
use crate::auth::rbac::Policy;

pub fn json_post<T: DeserializeOwned + Send>(
    context: RefContext,
) -> impl Filter<Extract = (T, RefContext), Error = Rejection> + Clone {
    warp::post().and(warp::body::json()).and(context.extract())
}

pub fn auth_json_post<T: DeserializeOwned + Send>(
    condition: impl Into<PolicyCondition<Policy>>,
    context: RefContext,
) -> impl Filter<Extract = (T, RefContext), Error = Rejection> + Clone {
    warp::post()
        .and(check_permission(condition.into(), context))
        .and(warp::body::json())
        .and(context.extract())
}

pub fn auth_get(
    condition: impl Into<PolicyCondition<Policy>>,
    context: RefContext,
) -> impl Filter<Extract = (RefContext,), Error = Rejection> + Clone {
    warp::get()
        .and(check_permission(condition.into(), context))
        .and(context.extract())
}
