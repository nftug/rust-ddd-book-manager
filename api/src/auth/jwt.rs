use infrastructure::config::OidcConfig;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{Algorithm, Validation, decode, decode_header};

use crate::auth::OidcAuthError;
use crate::auth::{claims::KeycloakClaims, jwks};

pub async fn decode_and_validate_token(
    token: &str,
    oidc: &OidcConfig,
) -> Result<KeycloakClaims, OidcAuthError> {
    let header = decode_header(token).map_err(|e| OidcAuthError::InvalidToken(e.to_string()))?;
    if header.alg != Algorithm::RS256 {
        return Err(OidcAuthError::InvalidToken("invalid algorithm".to_string()));
    }

    let kid = header
        .kid
        .ok_or(OidcAuthError::InvalidToken("missing kid".to_string()))?;
    let decoding_key = jwks::get_decoding_key(&oidc.authority, &kid).await?;

    let validation = build_validation(oidc);

    let token_data = decode::<KeycloakClaims>(token, &decoding_key, &validation).map_err(|e| {
        match e.kind() {
            ErrorKind::ExpiredSignature => OidcAuthError::Expired,
            _ => OidcAuthError::InvalidToken(e.to_string()),
        }
    })?;

    Ok(token_data.claims)
}

fn build_validation(oidc: &OidcConfig) -> Validation {
    let mut validation = Validation::new(Algorithm::RS256);

    if let Some(aud) = &oidc.audience {
        validation.set_audience(&[aud.as_str()]);
    } else {
        validation.validate_aud = false;
    }

    validation.iss = Some(vec![oidc.authority.clone()].into_iter().collect());

    validation
}
