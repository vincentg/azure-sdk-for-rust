#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents the properties of a send message request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMessageRequest {
    #[doc = "The sender's phone number in E.164 format that is owned by the authenticated account."]
    pub from: String,
    #[doc = "The recipient's phone number in E.164 format. In this version, a minimum of 1 and upto 100 recipients in the list are supported."]
    #[serde(rename = "smsRecipients")]
    pub sms_recipients: Vec<SmsRecipient>,
    #[doc = "The contents of the message that will be sent to the recipient. The allowable content is defined by RFC 5724."]
    pub message: String,
    #[doc = "Optional configuration for sending SMS messages."]
    #[serde(rename = "smsSendOptions", default, skip_serializing_if = "Option::is_none")]
    pub sms_send_options: Option<SmsSendOptions>,
}
impl SendMessageRequest {
    pub fn new(from: String, sms_recipients: Vec<SmsRecipient>, message: String) -> Self {
        Self {
            from,
            sms_recipients,
            message,
            sms_send_options: None,
        }
    }
}
#[doc = "Recipient details for sending SMS messages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsRecipient {
    #[doc = "The recipient's phone number in E.164 format."]
    pub to: String,
    #[doc = "If specified, the client directs that the request is repeatable; that is, the client can make the request multiple times with the same Repeatability-Request-ID and get back an appropriate response without the server executing the request multiple times. The value of the Repeatability-Request-ID is an opaque string representing a client-generated, 36-character hexadecimal case-insensitive encoding of a UUID (GUID), identifier for the request."]
    #[serde(rename = "repeatabilityRequestId", default, skip_serializing_if = "Option::is_none")]
    pub repeatability_request_id: Option<String>,
    #[doc = "MUST be sent by clients to specify that a request is repeatable. Repeatability-First-Sent is used to specify the date and time at which the request was first created.eg- Tue, 26 Mar 2019 16:06:51 GMT"]
    #[serde(rename = "repeatabilityFirstSent", default, skip_serializing_if = "Option::is_none")]
    pub repeatability_first_sent: Option<String>,
}
impl SmsRecipient {
    pub fn new(to: String) -> Self {
        Self {
            to,
            repeatability_request_id: None,
            repeatability_first_sent: None,
        }
    }
}
#[doc = "Optional configuration for sending SMS messages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsSendOptions {
    #[doc = "Enable this flag to receive a delivery report for this message on the Azure Resource EventGrid."]
    #[serde(rename = "enableDeliveryReport")]
    pub enable_delivery_report: bool,
    #[doc = "Use this field to provide metadata that will then be sent back in the corresponding Delivery Report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
impl SmsSendOptions {
    pub fn new(enable_delivery_report: bool) -> Self {
        Self {
            enable_delivery_report,
            tag: None,
        }
    }
}
#[doc = "Response for a successful or multi status send Sms request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsSendResponse {
    pub value: Vec<SmsSendResponseItem>,
}
impl SmsSendResponse {
    pub fn new(value: Vec<SmsSendResponseItem>) -> Self {
        Self { value }
    }
}
#[doc = "Response for a single recipient."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsSendResponseItem {
    #[doc = "The recipient's phone number in E.164 format."]
    pub to: String,
    #[doc = "The identifier of the outgoing Sms message. Only present if message processed."]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "HTTP Status code."]
    #[serde(rename = "httpStatusCode")]
    pub http_status_code: i64,
    #[doc = "The result of a repeatable request with one of the case-insensitive values accepted or rejected."]
    #[serde(rename = "repeatabilityResult", default, skip_serializing_if = "Option::is_none")]
    pub repeatability_result: Option<sms_send_response_item::RepeatabilityResult>,
    #[doc = "Indicates if the message is processed successfully or not."]
    pub successful: bool,
    #[doc = "Optional error message in case of 4xx/5xx/repeatable errors."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl SmsSendResponseItem {
    pub fn new(to: String, http_status_code: i64, successful: bool) -> Self {
        Self {
            to,
            message_id: None,
            http_status_code,
            repeatability_result: None,
            successful,
            error_message: None,
        }
    }
}
pub mod sms_send_response_item {
    use super::*;
    #[doc = "The result of a repeatable request with one of the case-insensitive values accepted or rejected."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RepeatabilityResult {
        #[serde(rename = "accepted")]
        Accepted,
        #[serde(rename = "rejected")]
        Rejected,
    }
}
