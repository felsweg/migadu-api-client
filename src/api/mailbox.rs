use crate::Parser;
use chrono::{prelude::*, serde::ts_seconds_option, ParseError};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Default)]
pub struct Mailbox {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_part: Option<String>, // Create/Read Only

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>, // Read Only

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>, // Read Only

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[clap(long)]
    pub is_internal: bool,

    #[clap(long)]
    pub may_send: bool,

    #[clap(long)]
    pub may_receive: bool,

    #[clap(long)]
    pub may_access_imap: bool,

    #[clap(long)]
    pub may_access_pop3: bool,

    #[clap(long)]
    pub may_access_managesieve: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_method: Option<String>, // Predefined Values

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>, // Create/Update, Only

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_recovery_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_action: Option<String>, // Predefined Values

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_aggressiveness: Option<String>, // Predefined Values

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_denylist: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_allowlist: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_denylist: Option<String>,

    #[clap(long)]
    pub autorespond_active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorespond_subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorespond_body: Option<String>,

    #[clap(parse(try_from_str = parse_date))]
    #[serde(with = "ts_seconds_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorespond_expires_on: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_active: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_plain_body: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_html_body: Option<String>,
}

/// Parses `input` as [`Date`]
fn parse_date(input: &str) -> Result<DateTime<Utc>, ParseError> {
    Ok(input.parse::<DateTime<Utc>>()?)
}
