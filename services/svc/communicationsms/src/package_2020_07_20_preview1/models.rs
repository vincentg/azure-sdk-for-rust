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
    #[doc = "The recipients' phone number in E.164 format. In this version, only one recipient in the list is supported."]
    pub to: Vec<String>,
    #[doc = "The contents of the message that will be sent to the recipient. The allowable content is defined by RFC 5724."]
    pub message: String,
    #[doc = "Optional configuration for sending SMS messages"]
    #[serde(rename = "sendSmsOptions", default, skip_serializing_if = "Option::is_none")]
    pub send_sms_options: Option<SendSmsOptions>,
}
impl SendMessageRequest {
    pub fn new(from: String, to: Vec<String>, message: String) -> Self {
        Self {
            from,
            to,
            message,
            send_sms_options: None,
        }
    }
}
#[doc = "Optional configuration for sending SMS messages"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SendSmsOptions {
    #[doc = "Enable this flag to receive a delivery report for this message on the Azure Resource EventGrid"]
    #[serde(rename = "enableDeliveryReport", default, skip_serializing_if = "Option::is_none")]
    pub enable_delivery_report: Option<bool>,
}
impl SendSmsOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for a successful send Sms request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SendSmsResponse {
    #[doc = "The identifier of the outgoing SMS message"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}
impl SendSmsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
