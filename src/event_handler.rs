use std::str::FromStr;

use base64::{Engine, prelude::BASE64_STANDARD};
use futures::future::join_all;
use reqwest::{Method, Request, Url};
use serenity::{
    all::{Message, Ready},
    async_trait,
    prelude::*,
};
use tracing::{error, info, info_span, warn};

use crate::{
    errors::DiscAgentError,
    schemas::{agent::Attachment, broker::QueueInfo, dependencies::Dependencies},
};

pub struct Handler {
    deps: Dependencies,
}

impl Handler {
    pub fn new(deps: Dependencies) -> Self {
        Self { deps: deps }
    }

    async fn get_attachment(&self, msg: &Message) -> Vec<Result<Attachment, DiscAgentError>> {
        info!("Getting attachment...");
        let tasks = msg.attachments.clone().into_iter().map(|attach| {
            let client = self.deps.http_client.clone();

            async move {
                let req = Request::new(Method::GET, Url::from_str(&attach.url).unwrap());
                let res = client.send(req).await?;
                let content = res.bytes().await?;
                Ok(Attachment {
                    filename: attach.filename,
                    content: BASE64_STANDARD.encode(content),
                    media_type: attach.content_type,
                })
            }
        });

        let attachments = join_all(tasks).await;
        attachments
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!(username = ready.user.name, "The bot is connected!");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.current_user().id {
            return;
        }

        let message_content = match msg.content.is_empty() {
            false => Some(msg.content.clone()),
            true => None,
        };
        let guild_id = msg.guild_id.map(|guild_id| guild_id.to_string());

        let span = info_span!(
            "event_message",
            message = message_content,
            author_name = msg.author.name,
            author_id = msg.author.id.to_string(),
            guild_id = guild_id
        );
        let _enter = span.enter();

        info!("Received message.");

        let mut attachments = Vec::new();
        for attachment in self.get_attachment(&msg).await {
            match attachment {
                Ok(attachment) => attachments.push(attachment),
                Err(err) => {
                    warn!(err = ?err, "Couldn\'t get attachment.");
                }
            }
        }

        let serialized_msg = crate::schemas::agent::MessageOut {
            platform: String::from("Discord"),
            author_id: msg.author.id.to_string(),
            author_name: Some(msg.author.name),
            group_id: guild_id,
            group_name: msg.guild_id.unwrap().name(ctx.cache),
            message: message_content,
            attachments: attachments,
        };

        if let Err(err) = self
            .deps
            .broker
            .publish_message(
                &QueueInfo::new(None, "disc-agent"),
                serde_json::to_string(&serialized_msg).unwrap().as_bytes(),
            )
            .await
        {
            error!(err = ?err, "Couldn\'t publish message.");
            return;
        }
    }
}
