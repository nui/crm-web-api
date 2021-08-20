use warp::{Filter, Rejection};

use tokidator::rbac::PolicyCondition;

use crate::app::context::RefContext;
use crate::app::rejection::TokenError;
use crate::auth::rbac::{AccessToken, Policy};
use crate::auth::BearerToken;

/// Check that request have valid access token and sufficient permissions.
pub fn check_permission(
    condition: PolicyCondition<Policy>,
    context: RefContext,
) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    parse_validate_access_token_enforce(condition, context)
        .map(drop)
        .untuple_one()
}

/// Check that request have valid access token.
pub fn extract_access_token(
    context: RefContext,
) -> impl Filter<Extract = (AccessToken,), Error = Rejection> + Clone {
    parse_validate_access_token_enforce(PolicyCondition::Nil, context)
}

/// Extract access token from `authorization` request header then check it.
fn parse_validate_access_token_enforce(
    condition: PolicyCondition<Policy>,
    context: RefContext,
) -> impl Filter<Extract = (AccessToken,), Error = Rejection> + Clone {
    async fn check(
        token: BearerToken,
        condition: PolicyCondition<Policy>,
        context: RefContext,
    ) -> Result<AccessToken, Rejection> {
        context.auth.enforce(condition, token).map_err(|err| {
            // Reject this request
            warp::reject::custom(TokenError::from(err))
        })
    }
    warp::header::<BearerToken>("authorization")
        .and(warp::any().map(move || condition.clone()))
        .and(context.extract())
        .and_then(check)
}
