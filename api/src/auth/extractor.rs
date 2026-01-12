use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};

use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{
    auth::{OidcAuthError, claims::OidcUserInfo, jwt::decode_and_validate_token},
    registry::AppRegistry,
};

impl FromRequestParts<AppRegistry> for OidcUserInfo {
    type Rejection = OidcAuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppRegistry,
    ) -> Result<Self, Self::Rejection> {
        oidc_from_request_parts(parts, state).await
    }
}

impl FromRequestParts<AppRegistry> for Option<OidcUserInfo> {
    type Rejection = OidcAuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppRegistry,
    ) -> Result<Self, Self::Rejection> {
        match oidc_from_request_parts(parts, state).await {
            Ok(oidc_user) => Ok(Some(oidc_user)),
            Err(OidcAuthError::MissingToken) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

async fn oidc_from_request_parts(
    parts: &mut Parts,
    state: &AppRegistry,
) -> Result<OidcUserInfo, OidcAuthError> {
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| OidcAuthError::MissingToken)?;

    let oidc_config = &state.config().oidc;
    let claims = decode_and_validate_token(bearer.token(), oidc_config).await?;

    OidcUserInfo::from_claims(claims)
}
