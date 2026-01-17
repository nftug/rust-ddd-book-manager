use jsonwebtoken::DecodingKey;
use serde::Deserialize;
use std::time::{Duration, Instant};
use tokio::sync::{OnceCell, RwLock};

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

const JWKS_TTL: Duration = Duration::from_secs(300);

#[derive(Debug, Default)]
struct JwksCache {
    keys: Vec<Jwk>,
    fetched_at: Option<Instant>,
}

impl JwksCache {
    fn is_fresh(&self) -> bool {
        match self.fetched_at {
            Some(at) => at.elapsed() < JWKS_TTL,
            None => false,
        }
    }
}

static JWKS_CACHE: OnceCell<RwLock<JwksCache>> = OnceCell::const_new();

pub async fn get_decoding_key(authority: &str, kid: &str) -> Result<DecodingKey, OidcAuthError> {
    let cache = jwks_cache().await;
    {
        let read_guard = cache.read().await;
        if read_guard.is_fresh()
            && let Some(jwk) = read_guard.keys.iter().find(|k| k.kid == kid)
        {
            return decoding_key_from_jwk(jwk);
        }
    }

    let keys = fetch_jwks(authority).await?;
    let jwk = keys
        .iter()
        .find(|k| k.kid == kid)
        .cloned()
        .ok_or(OidcAuthError::InvalidToken("missing key".to_string()))?;
    {
        let mut write_guard = cache.write().await;
        write_guard.keys = keys;
        write_guard.fetched_at = Some(Instant::now());
    }

    decoding_key_from_jwk(&jwk)
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

async fn jwks_cache() -> &'static RwLock<JwksCache> {
    JWKS_CACHE
        .get_or_init(|| async { RwLock::new(JwksCache::default()) })
        .await
}

fn decoding_key_from_jwk(jwk: &Jwk) -> Result<DecodingKey, OidcAuthError> {
    if jwk.kty != "RSA" {
        return Err(OidcAuthError::InvalidToken("invalid key type".to_string()));
    }

    DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|e| OidcAuthError::InvalidToken(e.to_string()))
}
