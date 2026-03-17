use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};

#[derive(Debug, Serialize)]
pub struct DiscoveryDocument {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri: String,
    pub scopes_supported: Vec<String>,
    pub grant_types_supported: Vec<String>,
}

#[derive(Debug)]
pub struct AuthorizationQuery {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub nonce: String,
    pub state: String,
    pub login_hint: Option<String>,
    pub mock_user: Option<String>,
}

impl AuthorizationQuery {
    pub fn parse(raw_query: Option<&str>) -> Result<Self> {
        let raw_query = raw_query.unwrap_or_default();
        let mut response_type = None;
        let mut client_id = None;
        let mut redirect_uri = None;
        let mut nonce = None;
        let mut state = None;
        let mut login_hint = None;
        let mut mock_user = None;

        for (key, value) in url::form_urlencoded::parse(raw_query.as_bytes()) {
            match key.as_ref() {
                "response_type" => response_type = Some(value.into_owned()),
                "client_id" => client_id = Some(value.into_owned()),
                "redirect_uri" => redirect_uri = Some(value.into_owned()),
                "nonce" => nonce = Some(value.into_owned()),
                "state" => state = Some(value.into_owned()),
                "login_hint" => login_hint = Some(value.into_owned()),
                "mock_user" => mock_user = Some(value.into_owned()),
                _ => {}
            }
        }

        Ok(Self {
            response_type: required_field("response_type", response_type)?,
            client_id: required_field("client_id", client_id)?,
            redirect_uri: required_field("redirect_uri", redirect_uri)?,
            nonce: required_field("nonce", nonce)?,
            state: required_field("state", state)?,
            login_hint,
            mock_user,
        })
    }
}

fn required_field(name: &str, value: Option<String>) -> Result<String> {
    value.ok_or_else(|| AppError::bad_request(format!("missing query parameter: {name}")))
}

#[derive(Debug, Deserialize)]
pub struct TokenForm {
    pub grant_type: String,
    pub redirect_uri: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
}
