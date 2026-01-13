#[derive(Debug)]
pub enum DiscAgentError {
    BrokerError(String),
    Lapin(lapin::Error),
    Reqwest(reqwest::Error),
    Serenity(serenity::Error),
}

impl From<lapin::Error> for DiscAgentError {
    fn from(err: lapin::Error) -> Self {
        Self::Lapin(err)
    }
}

impl From<reqwest::Error> for DiscAgentError {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<serenity::Error> for DiscAgentError {
    fn from(err: serenity::Error) -> Self {
        Self::Serenity(err)
    }
}
