use std::process::exit;
use std::sync::Arc;
use tracing::{error, info};

use serenity::all::GatewayIntents;
use serenity::prelude::*;

use crate::brokers::amqp::AMQPBroker;
use crate::clients::http_client::HTTPClient;
use crate::errors::DiscAgentError;
use crate::schemas::dependencies::Dependencies;

mod brokers;
mod clients;
mod errors;
mod event_handler;
mod schemas;

async fn get_client(deps: Dependencies) -> Result<Client, DiscAgentError> {
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set.");
    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES;

    info!("Starting client...");
    Ok(Client::builder(token, intents)
        .event_handler(event_handler::Handler::new(deps))
        .await?)
}

async fn get_dependencies() -> Result<Dependencies, DiscAgentError> {
    let uri = std::env::var("AMQP_URI").expect("AMQP_URI not set.");

    let mut broker = AMQPBroker::new();

    broker.connect(&uri).await?;

    Ok(Dependencies {
        broker: Arc::new(broker),
        http_client: Arc::new(HTTPClient::new()),
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let deps = match get_dependencies().await {
        Ok(deps) => deps,
        Err(err) => {
            error!(err = ?err, "Couldn\'t get dependencies.");
            exit(1);
        }
    };

    let mut client = match get_client(deps).await {
        Ok(client) => client,
        Err(err) => {
            error!(err = ?err, "Couldn\'t start the client.");
            exit(1);
        }
    };

    if let Err(err) = client.start().await {
        error!(err = ?err, "Couldn\'t start the client.");
        exit(1);
    }
}
