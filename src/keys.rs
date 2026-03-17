use std::path::Path;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey, LineEnding};
use rsa::traits::PublicKeyParts;
use rsa::{RsaPrivateKey, RsaPublicKey};

use crate::config;
use crate::error::{AppError, Result};

#[derive(Clone)]
pub struct SigningKeyMaterial {
    pub key_id: String,
    pub encoding_key: jsonwebtoken::EncodingKey,
    pub jwk: Jwk,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Clone)]
pub struct Jwk {
    pub kty: String,
    pub kid: String,
    pub use_: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

impl serde::Serialize for Jwk {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Jwk", 6)?;
        state.serialize_field("kty", &self.kty)?;
        state.serialize_field("kid", &self.kid)?;
        state.serialize_field("use", &self.use_)?;
        state.serialize_field("alg", &self.alg)?;
        state.serialize_field("n", &self.n)?;
        state.serialize_field("e", &self.e)?;
        state.end()
    }
}

pub async fn load_or_create(path: &Path) -> Result<SigningKeyMaterial> {
    let pem = match tokio::fs::read_to_string(path).await {
        Ok(existing) => existing,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            let private = generate_private_key()?;
            let pem = private.to_pkcs8_pem(LineEnding::LF)?.to_string();
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            tokio::fs::write(path, pem.as_bytes()).await?;
            pem
        }
        Err(err) => return Err(err.into()),
    };

    from_pem(config::key_id(), &pem)
}

fn from_pem(key_id: &str, pem: &str) -> Result<SigningKeyMaterial> {
    let private = RsaPrivateKey::from_pkcs8_pem(pem)?;
    let public = RsaPublicKey::from(&private);
    let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(pem.as_bytes())?;

    Ok(SigningKeyMaterial {
        key_id: key_id.to_string(),
        encoding_key,
        jwk: jwk_from_public_key(key_id, &public),
    })
}

fn generate_private_key() -> Result<RsaPrivateKey> {
    let mut rng = rand::thread_rng();
    RsaPrivateKey::new(&mut rng, 2048).map_err(|err| AppError::internal(err.to_string()))
}

fn jwk_from_public_key(key_id: &str, public: &RsaPublicKey) -> Jwk {
    Jwk {
        kty: "RSA".to_string(),
        kid: key_id.to_string(),
        use_: "sig".to_string(),
        alg: "RS256".to_string(),
        n: URL_SAFE_NO_PAD.encode(public.n().to_bytes_be()),
        e: URL_SAFE_NO_PAD.encode(public.e().to_bytes_be()),
    }
}
