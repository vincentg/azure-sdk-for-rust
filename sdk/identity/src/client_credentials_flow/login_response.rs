use azure_core::auth::AccessToken;
use serde::{de, Deserialize, Deserializer};
use time::OffsetDateTime;

#[derive(Debug, Clone, Deserialize)]
struct _LoginResponse {
    token_type: String,
    expires_in: u64,
    ext_expires_in: u64,
    expires_on: Option<String>,
    not_before: Option<String>,
    resource: Option<String>,
    access_token: String,
}

#[derive(Debug, Clone)]
pub struct LoginResponse {
    pub token_type: String,
    pub expires_in: u64,
    pub ext_expires_in: u64,
    pub expires_on: Option<OffsetDateTime>,
    pub not_before: Option<OffsetDateTime>,
    pub resource: Option<String>,
    pub access_token: AccessToken,
}

impl<'de> Deserialize<'de> for LoginResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let resp = _LoginResponse::deserialize(deserializer)?;
        LoginResponse::from_base_response(resp).map_err(de::Error::custom)
    }
}

impl LoginResponse {
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    fn from_base_response(r: _LoginResponse) -> Result<LoginResponse, std::num::ParseIntError> {
        let expires_on: Option<OffsetDateTime> = r.expires_on.map(|d| {
            OffsetDateTime::from_unix_timestamp(d.parse::<i64>().unwrap_or(0))
                .unwrap_or(OffsetDateTime::UNIX_EPOCH)
        });
        let not_before: Option<OffsetDateTime> = r.not_before.map(|d| {
            OffsetDateTime::from_unix_timestamp(d.parse::<i64>().unwrap_or(0))
                .unwrap_or(OffsetDateTime::UNIX_EPOCH)
        });

        Ok(LoginResponse {
            token_type: r.token_type,
            expires_in: r.expires_in,
            ext_expires_in: r.ext_expires_in,
            expires_on,
            not_before,
            resource: r.resource,
            access_token: AccessToken::new(r.access_token),
        })
    }
}
