use lapin::{Connection, options::BasicPublishOptions, protocol::basic::AMQPProperties};
use serenity::async_trait;
use tracing::{error, info};

use crate::{brokers::broker::Broker, errors::DiscAgentError, schemas::broker::QueueInfo};

#[derive(Default)]
pub struct AMQPBroker {
    conn: Option<Connection>,
}

impl AMQPBroker {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub async fn connect(&mut self, uri: &str) -> Result<(), DiscAgentError> {
        info!("Connecting to AMQP...");
        self.conn = Some(Connection::connect(uri, lapin::ConnectionProperties::default()).await?);

        info!("Connected to AMQP.");
        Ok(())
    }
}

#[async_trait]
impl Broker for AMQPBroker {
    async fn publish_message(
        &self,
        queue: &QueueInfo,
        msg: &[u8],
    ) -> Result<(), crate::errors::DiscAgentError> {
        let span = tracing::info_span!("publish_message", queue = ?queue, msg_size = msg.len());
        let _enter = span.enter();

        if self.conn.is_none() {
            error!("Couldn\'t publish message. No connection.");
            return Err(DiscAgentError::BrokerError(
                "Couldn\'t publish message. No connection.".to_string(),
            ));
        }

        info!("Publishing message...");
        let conn = self.conn.as_ref().unwrap();
        let channel = conn.create_channel().await?;

        channel
            .basic_publish(
                &queue.exchange,
                &queue.routing_key,
                BasicPublishOptions::default(),
                msg,
                AMQPProperties::default(),
            )
            .await?;

        info!("Published message.");
        Ok(())
    }
}
