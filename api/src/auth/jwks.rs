use jsonwebtoken::DecodingKey;
use serde::Deserialize;

use crate::auth::OidcAuthError;

#[derive(Debug, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize, Clone)]
struct Jwk {
    kid: String,
    kty: String,
    n: String,
    e: String,
}

pub async fn get_decoding_key(authority: &str, kid: &str) -> Result<DecodingKey, OidcAuthError> {
    let jwk = fetch_jwks(authority)
        .await?
        .into_iter()
        .find(|k| k.kid == kid)
        .ok_or(OidcAuthError::InvalidToken("missing key".to_string()))?;

    if jwk.kty != "RSA" {
        return Err(OidcAuthError::InvalidToken("invalid key type".to_string()));
    }

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|e| OidcAuthError::InvalidToken(e.to_string()))?;

    Ok(decoding_key)
}

async fn fetch_jwks(authority: &str) -> Result<Vec<Jwk>, OidcAuthError> {
    let base = authority.trim_end_matches('/');
    let url = format!("{}/protocol/openid-connect/certs", base);

    let resp = reqwest::get(url)
        .await
        .map_err(|_| OidcAuthError::JwksFetchError)?;

    if !resp.status().is_success() {
        return Err(OidcAuthError::JwksFetchError);
    }

    resp.json::<JwkSet>()
        .await
        .map(|jwk_set| jwk_set.keys)
        .map_err(|_| OidcAuthError::JwksFetchError)
}
