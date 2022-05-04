use color_eyre::Result;
use markov::Chain;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, io::Read};
use urlencoding::encode;
use uuid::Uuid;

use crate::{
    markov::{feed_post, ready_chain},
    parse_mention::{is_mention, strip_mention_content},
};

const BASE_URL: &str = "https://botsin.space/api/v1";

#[derive(Debug, Serialize, Deserialize)]
struct Auth {
    pub key: String,
    pub secret: String,
    pub token: String,
}

fn load_auth() -> Result<Auth> {
    let mut raw = String::new();
    fs::File::open("auth.json")?.read_to_string(&mut raw)?;

    Ok(serde_json::from_str(&raw)?)
}

#[derive(Debug)]
pub struct Bot {
    auth: Auth,
    pub chain: Chain<String>,
    client: Client,
}

/// Returns a url of the statuses section of the api with the content as a query parameter, urlencoded
fn gen_url(content: &str) -> Url {
    let content = encode(content);
    // I'm decently sure unwrap is justified here because we're supplying everything
    Url::parse(&format!("{BASE_URL}/statuses?status={content}")).unwrap()
}

impl Bot {
    pub fn try_load() -> Result<Self> {
        let auth = load_auth()?;
        let chain = ready_chain();
        let client = Client::new();

        Ok(Self {
            auth,
            chain,
            client,
        })
    }

    pub async fn post(&self, content: &str) -> Result<()> {
        self.client
            .post(gen_url(content))
            .header("Authorization", format!("Bearer {}", self.auth.token))
            .header("Idempotency-key", Uuid::new_v4().to_string())
            .send()
            .await?;

        Ok(())
    }

    pub async fn post_generated(&self) -> Result<()> {
        self.post(&self.chain.generate_str()).await?;

        Ok(())
    }

    fn feed_post(&mut self, post: &str) {
        feed_post(&mut self.chain, post);
    }

    async fn clear_notifs(&self) -> Result<()> {
        self.client
            .post(&format!("{BASE_URL}/notifications/clear"))
            .header("Authorization", format!("Bearer {}", self.auth.token))
            .send()
            .await?;

        Ok(())
    }

    pub async fn handle_notifs(&mut self) -> Result<()> {
        let returned: Value = self
            .client
            .get(&format!("{BASE_URL}/notifications"))
            .header("Authorization", format!("Bearer {}", self.auth.token))
            .send()
            .await?
            .json()
            .await?;

        for notif in returned.as_array().expect("oopsie") {
            if is_mention(notif) {
                let content = notif
                    .as_object()
                    .unwrap()
                    .get("status")
                    .unwrap()
                    .get("content")
                    .unwrap();
                let content = strip_mention_content(content.as_str().unwrap());

                self.feed_post(&content);
            }
        }

        self.clear_notifs().await?;
        Ok(())
    }
}
