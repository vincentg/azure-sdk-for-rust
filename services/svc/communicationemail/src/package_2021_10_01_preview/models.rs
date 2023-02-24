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
#[doc = "An object representing the email address and its display name"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailAddress {
    #[doc = "Email address."]
    pub email: String,
    #[doc = "Email display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl EmailAddress {
    pub fn new(email: String) -> Self {
        Self { email, display_name: None }
    }
}
#[doc = "Attachment to the email."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailAttachment {
    #[doc = "Name of the attachment"]
    pub name: String,
    #[doc = "The type of attachment file."]
    #[serde(rename = "attachmentType")]
    pub attachment_type: EmailAttachmentType,
    #[doc = "Base64 encoded contents of the attachment"]
    #[serde(rename = "contentBytesBase64")]
    pub content_bytes_base64: String,
}
impl EmailAttachment {
    pub fn new(name: String, attachment_type: EmailAttachmentType, content_bytes_base64: String) -> Self {
        Self {
            name,
            attachment_type,
            content_bytes_base64,
        }
    }
}
#[doc = "The type of attachment file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EmailAttachmentType")]
pub enum EmailAttachmentType {
    #[serde(rename = "avi")]
    Avi,
    #[serde(rename = "bmp")]
    Bmp,
    #[serde(rename = "doc")]
    Doc,
    #[serde(rename = "docm")]
    Docm,
    #[serde(rename = "docx")]
    Docx,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "one")]
    One,
    #[serde(rename = "pdf")]
    Pdf,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "ppsm")]
    Ppsm,
    #[serde(rename = "ppsx")]
    Ppsx,
    #[serde(rename = "ppt")]
    Ppt,
    #[serde(rename = "pptm")]
    Pptm,
    #[serde(rename = "pptx")]
    Pptx,
    #[serde(rename = "pub")]
    Pub,
    #[serde(rename = "rpmsg")]
    Rpmsg,
    #[serde(rename = "rtf")]
    Rtf,
    #[serde(rename = "tif")]
    Tif,
    #[serde(rename = "txt")]
    Txt,
    #[serde(rename = "vsd")]
    Vsd,
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "wma")]
    Wma,
    #[serde(rename = "xls")]
    Xls,
    #[serde(rename = "xlsb")]
    Xlsb,
    #[serde(rename = "xlsm")]
    Xlsm,
    #[serde(rename = "xlsx")]
    Xlsx,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EmailAttachmentType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EmailAttachmentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EmailAttachmentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Avi => serializer.serialize_unit_variant("EmailAttachmentType", 0u32, "avi"),
            Self::Bmp => serializer.serialize_unit_variant("EmailAttachmentType", 1u32, "bmp"),
            Self::Doc => serializer.serialize_unit_variant("EmailAttachmentType", 2u32, "doc"),
            Self::Docm => serializer.serialize_unit_variant("EmailAttachmentType", 3u32, "docm"),
            Self::Docx => serializer.serialize_unit_variant("EmailAttachmentType", 4u32, "docx"),
            Self::Gif => serializer.serialize_unit_variant("EmailAttachmentType", 5u32, "gif"),
            Self::Jpeg => serializer.serialize_unit_variant("EmailAttachmentType", 6u32, "jpeg"),
            Self::Mp3 => serializer.serialize_unit_variant("EmailAttachmentType", 7u32, "mp3"),
            Self::One => serializer.serialize_unit_variant("EmailAttachmentType", 8u32, "one"),
            Self::Pdf => serializer.serialize_unit_variant("EmailAttachmentType", 9u32, "pdf"),
            Self::Png => serializer.serialize_unit_variant("EmailAttachmentType", 10u32, "png"),
            Self::Ppsm => serializer.serialize_unit_variant("EmailAttachmentType", 11u32, "ppsm"),
            Self::Ppsx => serializer.serialize_unit_variant("EmailAttachmentType", 12u32, "ppsx"),
            Self::Ppt => serializer.serialize_unit_variant("EmailAttachmentType", 13u32, "ppt"),
            Self::Pptm => serializer.serialize_unit_variant("EmailAttachmentType", 14u32, "pptm"),
            Self::Pptx => serializer.serialize_unit_variant("EmailAttachmentType", 15u32, "pptx"),
            Self::Pub => serializer.serialize_unit_variant("EmailAttachmentType", 16u32, "pub"),
            Self::Rpmsg => serializer.serialize_unit_variant("EmailAttachmentType", 17u32, "rpmsg"),
            Self::Rtf => serializer.serialize_unit_variant("EmailAttachmentType", 18u32, "rtf"),
            Self::Tif => serializer.serialize_unit_variant("EmailAttachmentType", 19u32, "tif"),
            Self::Txt => serializer.serialize_unit_variant("EmailAttachmentType", 20u32, "txt"),
            Self::Vsd => serializer.serialize_unit_variant("EmailAttachmentType", 21u32, "vsd"),
            Self::Wav => serializer.serialize_unit_variant("EmailAttachmentType", 22u32, "wav"),
            Self::Wma => serializer.serialize_unit_variant("EmailAttachmentType", 23u32, "wma"),
            Self::Xls => serializer.serialize_unit_variant("EmailAttachmentType", 24u32, "xls"),
            Self::Xlsb => serializer.serialize_unit_variant("EmailAttachmentType", 25u32, "xlsb"),
            Self::Xlsm => serializer.serialize_unit_variant("EmailAttachmentType", 26u32, "xlsm"),
            Self::Xlsx => serializer.serialize_unit_variant("EmailAttachmentType", 27u32, "xlsx"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Content of the email."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailContent {
    #[doc = "Subject of the email message"]
    pub subject: String,
    #[doc = "Plain text version of the email message."]
    #[serde(rename = "plainText", default, skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<String>,
    #[doc = "Html version of the email message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
}
impl EmailContent {
    pub fn new(subject: String) -> Self {
        Self {
            subject,
            plain_text: None,
            html: None,
        }
    }
}
#[doc = "Custom header for email."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailCustomHeader {
    #[doc = "Header name."]
    pub name: String,
    #[doc = "Header value."]
    pub value: String,
}
impl EmailCustomHeader {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "The importance type for the email."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EmailImportance")]
pub enum EmailImportance {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "low")]
    Low,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EmailImportance {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EmailImportance {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EmailImportance {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::High => serializer.serialize_unit_variant("EmailImportance", 0u32, "high"),
            Self::Normal => serializer.serialize_unit_variant("EmailImportance", 1u32, "normal"),
            Self::Low => serializer.serialize_unit_variant("EmailImportance", 2u32, "low"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for EmailImportance {
    fn default() -> Self {
        Self::Normal
    }
}
#[doc = "Message payload for sending an email"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailMessage {
    #[doc = "Custom email headers to be passed."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<EmailCustomHeader>,
    #[doc = "Sender email address from a verified domain."]
    pub sender: String,
    #[doc = "Content of the email."]
    pub content: EmailContent,
    #[doc = "The importance type for the email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importance: Option<EmailImportance>,
    #[doc = "Recipients of the email"]
    pub recipients: EmailRecipients,
    #[doc = "list of attachments"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<EmailAttachment>,
    #[doc = "Email addresses where recipients' replies will be sent to."]
    #[serde(rename = "replyTo", default, skip_serializing_if = "Vec::is_empty")]
    pub reply_to: Vec<EmailAddress>,
    #[doc = "Indicates whether user engagement tracking should be disabled for this request if the resource-level user engagement tracking setting was already enabled in the control plane."]
    #[serde(rename = "disableUserEngagementTracking", default, skip_serializing_if = "Option::is_none")]
    pub disable_user_engagement_tracking: Option<bool>,
}
impl EmailMessage {
    pub fn new(sender: String, content: EmailContent, recipients: EmailRecipients) -> Self {
        Self {
            headers: Vec::new(),
            sender,
            content,
            importance: None,
            recipients,
            attachments: Vec::new(),
            reply_to: Vec::new(),
            disable_user_engagement_tracking: None,
        }
    }
}
#[doc = "Recipients of the email"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailRecipients {
    #[doc = "Email To recipients"]
    pub to: Vec<EmailAddress>,
    #[doc = "Email CC recipients"]
    #[serde(rename = "CC", default, skip_serializing_if = "Vec::is_empty")]
    pub cc: Vec<EmailAddress>,
    #[doc = "Email BCC recipients"]
    #[serde(rename = "bCC", default, skip_serializing_if = "Vec::is_empty")]
    pub b_cc: Vec<EmailAddress>,
}
impl EmailRecipients {
    pub fn new(to: Vec<EmailAddress>) -> Self {
        Self {
            to,
            cc: Vec::new(),
            b_cc: Vec::new(),
        }
    }
}
#[doc = "The type indicating the status of a request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SendStatus")]
pub enum SendStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "outForDelivery")]
    OutForDelivery,
    #[serde(rename = "dropped")]
    Dropped,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SendStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SendStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SendStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Queued => serializer.serialize_unit_variant("SendStatus", 0u32, "queued"),
            Self::OutForDelivery => serializer.serialize_unit_variant("SendStatus", 1u32, "outForDelivery"),
            Self::Dropped => serializer.serialize_unit_variant("SendStatus", 2u32, "dropped"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Status of an email message that was sent previously."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendStatusResult {
    #[doc = "System generated id of an email message sent."]
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[doc = "The type indicating the status of a request."]
    pub status: SendStatus,
}
impl SendStatusResult {
    pub fn new(message_id: String, status: SendStatus) -> Self {
        Self { message_id, status }
    }
}
