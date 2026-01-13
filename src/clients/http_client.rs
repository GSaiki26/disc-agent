use std::env::var;

use reqwest::{Client, Request, Response};
use tokio::sync::Semaphore;
use tracing::{debug, info_span};

pub struct HTTPClient {
    pub client: Client,
    sem: Option<Semaphore>,
}

impl HTTPClient {
    pub fn new() -> Self {
        let concurrency = var("HTTP_CLIENT_CONCURRENCY").map(|concurrency| {
            concurrency
                .parse::<usize>()
                .expect("HTTP_CLIENT_CONCURRENCY must be a number.")
        });

        Self {
            client: Client::new(),
            sem: match concurrency {
                Ok(concurrency) => Some(Semaphore::new(concurrency)),
                Err(_) => None,
            },
        }
    }

    pub async fn send(&self, req: Request) -> reqwest::Result<Response> {
        let _sem = match &self.sem {
            Some(sem) => Some(sem.acquire().await.unwrap()),
            None => None,
        };

        let span = info_span!("http_client_send",
            method =? req.method(),
            url =? req.url()
        );
        let _guard = span.enter();

        debug!(body =? req.body(), headers =? req.headers(), "Sending request...");

        match self.client.execute(req).await {
            Ok(res) => {
                debug!(
                    status =? res.status(),
                    "The request has been successfully completed."
                );
                Ok(res)
            }
            Err(err) => {
                debug!(err =? err, "The request has failed.");
                Err(err)
            }
        }
    }
}
