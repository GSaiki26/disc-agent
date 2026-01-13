#[derive(Clone, Debug)]
pub struct QueueInfo {
    pub exchange: String,
    pub routing_key: String,
}

impl QueueInfo {
    pub fn new(exchange: Option<String>, routing_key: &str) -> Self {
        let exchange = exchange.unwrap_or_else(|| String::new());
        Self {
            exchange: exchange,
            routing_key: routing_key.to_string(),
        }
    }
}
