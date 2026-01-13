use serenity::async_trait;

use crate::{errors::DiscAgentError, schemas::broker::QueueInfo};

#[async_trait]
pub trait Broker: Send + Sync {
    async fn publish_message(&self, queue: &QueueInfo, msg: &[u8]) -> Result<(), DiscAgentError>;
}
