#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The Communication Services error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationError {
    #[doc = "The error code."]
    pub code: String,
    #[doc = "The error message."]
    pub message: String,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Further details about specific errors that led to this error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CommunicationError>,
    #[doc = "The Communication Services error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<CommunicationError>>,
}
impl CommunicationError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "The Communication Services error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationErrorResponse {
    #[doc = "The Communication Services error."]
    pub error: CommunicationError,
}
impl CommunicationErrorResponse {
    pub fn new(error: CommunicationError) -> Self {
        Self { error }
    }
}
#[doc = "A communication identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationIdentity {
    #[doc = "Identifier of the identity."]
    pub id: String,
}
impl CommunicationIdentity {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "An access token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationIdentityAccessToken {
    #[doc = "The access token issued for the identity."]
    pub token: String,
    #[doc = "The expiry time of the token."]
    #[serde(rename = "expiresOn", with = "azure_core::date::rfc3339")]
    pub expires_on: time::OffsetDateTime,
}
impl CommunicationIdentityAccessToken {
    pub fn new(token: String, expires_on: time::OffsetDateTime) -> Self {
        Self { token, expires_on }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationIdentityAccessTokenRequest {
    #[doc = "List of scopes attached to the token."]
    pub scopes: Vec<CommunicationIdentityTokenScope>,
}
impl CommunicationIdentityAccessTokenRequest {
    pub fn new(scopes: Vec<CommunicationIdentityTokenScope>) -> Self {
        Self { scopes }
    }
}
#[doc = "A communication identity with access token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationIdentityAccessTokenResult {
    #[doc = "A communication identity."]
    pub identity: CommunicationIdentity,
    #[doc = "An access token."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<CommunicationIdentityAccessToken>,
}
impl CommunicationIdentityAccessTokenResult {
    pub fn new(identity: CommunicationIdentity) -> Self {
        Self {
            identity,
            access_token: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationIdentityCreateRequest {
    #[doc = "Also create access token for the created identity."]
    #[serde(rename = "createTokenWithScopes", default, skip_serializing_if = "Vec::is_empty")]
    pub create_token_with_scopes: Vec<CommunicationIdentityTokenScope>,
}
impl CommunicationIdentityCreateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of scopes for an access token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CommunicationIdentityTokenScope")]
pub enum CommunicationIdentityTokenScope {
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "voip")]
    Voip,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CommunicationIdentityTokenScope {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CommunicationIdentityTokenScope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CommunicationIdentityTokenScope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Chat => serializer.serialize_unit_variant("CommunicationIdentityTokenScope", 0u32, "chat"),
            Self::Voip => serializer.serialize_unit_variant("CommunicationIdentityTokenScope", 1u32, "voip"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsUserExchangeTokenRequest {
    #[doc = "Azure AD access token of a Teams User to acquire a new Communication Identity access token."]
    pub token: String,
    #[doc = "Client ID of an Azure AD application to be verified against the appid claim in the Azure AD access token."]
    #[serde(rename = "appId")]
    pub app_id: String,
    #[doc = "Object ID of an Azure AD user (Teams User) to be verified against the oid claim in the Azure AD access token."]
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl TeamsUserExchangeTokenRequest {
    pub fn new(token: String, app_id: String, user_id: String) -> Self {
        Self { token, app_id, user_id }
    }
}
