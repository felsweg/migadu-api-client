use serde::{Deserialize, Serialize};

use crate::Parser;

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Alias {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_part: Option<String>, // 	Create/Read Only

    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>, //Read Only

    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>, // 	Read Only

    #[clap(long)]
    is_internal: bool,
    destinations: Vec<String>,
}
