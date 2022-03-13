pub mod alias;
pub mod identity;
pub mod mailbox;
pub mod rewrite;

// re-export all modules
pub use alias::*;
use clap::Subcommand;
pub use identity::*;
pub use mailbox::*;
use reqwest::Response;
pub use rewrite::*;

pub use clap::Parser;
use serde::{Deserialize, Serialize};

/// This is the migadu app for all parameters
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Migadu {
    #[clap(subcommand)]
    #[serde(flatten)]
    pub api: API,

    #[clap(flatten)]
    pub credentials: Credentials,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Credentials {
    #[clap(long, short = 'u')]
    pub user: String,

    #[clap(long, short = 'p')]
    pub pass: String,

    #[clap(long, short = 'd')]
    pub domain: String,
}

#[derive(Subcommand, Debug, Serialize, Deserialize, Clone)]
pub enum Action {
    INDEX,
    SHOW,
    CREATE,
    UPDATE,
    DELETE,
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
// #[serde(untagged)]
pub enum API {
    Mailboxes {
        #[clap(flatten)]
        data: Mailbox,

        #[clap(long)]
        extra: Option<String>,

        #[clap(subcommand)]
        action: Action,
    },
    Identities {
        #[clap(flatten)]
        data: Identity,

        #[clap(long)]
        extra: Option<String>,

        #[clap(subcommand)]
        action: Action,
    },

    Aliases {
        #[clap(flatten)]
        data: Alias,

        #[clap(long)]
        extra: Option<String>,

        #[clap(subcommand)]
        action: Action,
    },

    Rewrites {
        #[clap(flatten)]
        data: Rewrite,

        #[clap(long)]
        extra: Option<String>,

        #[clap(subcommand)]
        action: Action,
    },
}

impl ToString for API {
    fn to_string(&self) -> String {
        match &self {
            API::Mailboxes { extra, .. } => {
                format!(
                    "{}{}",
                    "mailboxes".to_string(),
                    extra.as_ref().map_or("".to_string(), |a| {
                        let mut out = "/".to_string();
                        out.push_str(a.as_str());
                        out
                    })
                )
            }
            API::Aliases { extra, .. } => {
                format!(
                    "{}{}",
                    "aliases".to_string(),
                    extra.as_ref().map_or("".to_string(), |a| {
                        let mut out = "/".to_string();
                        out.push_str(a.as_str());
                        out
                    })
                )
            }
            API::Identities { extra, .. } => {
                format!(
                    "{}{}",
                    "identities".to_string(),
                    extra.as_ref().map_or("".to_string(), |a| {
                        let mut out = "/".to_string();
                        out.push_str(a.as_str());
                        out
                    })
                )
            }
            API::Rewrites { extra, .. } => {
                format!(
                    "{}{}",
                    "rewrites".to_string(),
                    extra.as_ref().map_or("".to_string(), |a| {
                        let mut out = "/".to_string();
                        out.push_str(a.as_str());
                        out
                    })
                )
            }
        }
    }
}

impl Migadu {
    /// Returns the API endpoint to be used
    pub fn endpoint(&self, endpoint: String) -> String {
        let api_name = format!("{}", &self.api.to_string().to_lowercase());
        format!("{}/{}/{}", endpoint, self.credentials.domain, api_name)
    }

    /// Returns the action to be run
    pub fn action(&self) -> &Action {
        match &self.api {
            API::Mailboxes {
                action: command, ..
            } => command,
            API::Aliases {
                action: command, ..
            } => command,
            API::Identities {
                action: command, ..
            } => command,
            API::Rewrites {
                action: command, ..
            } => command,
        }
    }

    /// Serializes the underlying data as json string
    pub fn json_data(&self) -> Result<String, serde_json::Error> {
        match &self.api {
            API::Mailboxes { data, .. } => serde_json::to_string(&data),
            API::Aliases { data, .. } => serde_json::to_string(&data),
            API::Identities { data, .. } => serde_json::to_string(&data),
            API::Rewrites { data, .. } => serde_json::to_string(&data),
        }
    }

    /// Returns the provided used
    pub fn user(&self) -> &String {
        &self.credentials.user
    }

    /// Returns the provided API token
    pub fn pass(&self) -> &String {
        &self.credentials.pass
    }

    /// Returns true, if the specific item  has been provided
    pub fn has_extra(&self) -> bool {
        match &self.api {
            API::Mailboxes { extra, .. } => extra.is_some(),
            API::Aliases { extra, .. } => extra.is_some(),
            API::Identities { extra, .. } => extra.is_some(),
            API::Rewrites { extra, .. } => extra.is_some(),
        }
    }

    /// api call: lists all for action
    pub async fn index(&self, endpoint: String) -> Result<Response, reqwest::Error> {
        let uri: reqwest::Url = self.endpoint(endpoint).parse().unwrap();

        let client = reqwest::Client::new();
        client
            .get(uri)
            .basic_auth(self.user(), Some(self.pass()))
            .send()
            .await
    }

    /// api call: shows specific item
    pub async fn show(&self, endpoint: String) -> Result<Response, reqwest::Error> {
        // extra is required
        assert!(self.has_extra(), "Must provide specific update item");

        let uri: reqwest::Url = self.endpoint(endpoint).parse().unwrap();

        let client = reqwest::Client::new();
        client
            .get(uri)
            .basic_auth(self.user(), Some(self.pass()))
            .send()
            .await
    }

    /// api call: create new item
    pub async fn create(&self, endpoint: String) -> Result<Response, reqwest::Error> {
        let uri: reqwest::Url = self.endpoint(endpoint).parse().unwrap();

        let client = reqwest::Client::new();

        let json_body = self.json_data().expect("Could not unwrap json data");
        client
            .post(uri)
            .body(json_body)
            .basic_auth(self.user(), Some(self.pass()))
            .send()
            .await
    }

    /// api call: updates specific item
    pub async fn update(&self, endpoint: String) -> Result<Response, reqwest::Error> {
        // extra is required
        assert!(self.has_extra(), "Must provide specific update item");

        let uri: reqwest::Url = self.endpoint(endpoint).parse().unwrap();

        let client = reqwest::Client::new();

        let json_body = self.json_data().expect("Could not unwrap json data");

        Ok(client
            .post(uri)
            .body(json_body)
            .basic_auth(self.user(), Some(self.pass()))
            .send()
            .await?)
    }

    /// api call: deletes specific item
    pub async fn delete(&self, endpoint: String) -> Result<Response, reqwest::Error> {
        // extra is required
        assert!(self.has_extra(), "Must provide specific update item");

        let uri: reqwest::Url = self.endpoint(endpoint).parse().unwrap();

        let client = reqwest::Client::new();

        let json_body = self.json_data().expect("Could not unwrap json data");
        client
            .post(uri)
            .body(json_body)
            .basic_auth(self.user(), Some(self.pass()))
            .send()
            .await
    }

    /// calls api
    pub async fn run(&self, endpoint: String) -> Result<Response, reqwest::Error> {
        match self.action() {
            Action::CREATE => self.create(endpoint).await,
            Action::DELETE => self.delete(endpoint).await,
            Action::INDEX => self.index(endpoint).await,
            Action::UPDATE => self.update(endpoint).await,
            Action::SHOW => self.show(endpoint).await,
        }
    }
}
