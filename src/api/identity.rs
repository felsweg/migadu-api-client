use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Identity {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_part: Option<String>, // Read Only

    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>, //Read Only

    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>, //	Read Only

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[clap(long)]
    may_send: bool,

    #[clap(long)]
    may_receive: bool,

    #[clap(long)]
    may_access_imap: bool,

    #[clap(long)]
    may_access_pop3: bool,

    #[clap(long)]
    may_access_managesieve: bool,
    password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    footer_active: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    footer_plain_body: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    footer_html_body: Option<String>,
}
