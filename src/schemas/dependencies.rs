use std::sync::Arc;

use crate::{brokers::broker::Broker, clients::http_client::HTTPClient};

pub struct Dependencies {
    pub broker: Arc<dyn Broker>,
    pub http_client: Arc<HTTPClient>,
}
