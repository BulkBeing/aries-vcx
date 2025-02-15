use crate::a2a_message;
use crate::error::prelude::*;
use crate::a2a::message_type::MessageType;
use crate::a2a::{A2AMessage, MessageId};
use crate::attachment::Attachments;
use crate::did_doc::service_resolvable::ServiceResolvable;
use crate::mime_type::MimeType;
use crate::timing::Timing;
use crate::out_of_band::GoalCode;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct OutOfBandInvitation {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_code: Option<GoalCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<Vec<MimeType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake_protocols: Option<Vec<MessageType>>,
    pub services: Vec<ServiceResolvable>,
    #[serde(rename = "requests~attach")]
    pub requests_attach: Attachments,
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}

a2a_message!(OutOfBandInvitation);

impl OutOfBandInvitation {
    pub fn to_string(&self) -> String {
        json!(self).to_string()
    }

    pub fn from_string(oob_data: &str) -> MessagesResult<OutOfBandInvitation> {
        serde_json::from_str(oob_data).map_err(|err| {
            MessagesError::from_msg(
                MesssagesErrorKind::InvalidJson,
                format!("Cannot deserialize out of band message: {:?}", err),
            )
        })
    }
}
