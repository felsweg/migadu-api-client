use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Rewrite {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>, // 	Read Only

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>, // 	Slug

    #[serde(skip_serializing_if = "Option::is_none")]
    local_part_rule: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    order_num: Option<usize>,

    destinations: Vec<String>,
}
