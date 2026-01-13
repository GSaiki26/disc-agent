#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageOut {
    pub platform: String,            // "Discord",
    pub author_id: String,           // "1234",
    pub author_name: Option<String>, // "Saiki",
    pub group_id: Option<String>,    // "4321",
    pub group_name: Option<String>,  // "los_manos",
    pub message: Option<String>,     // "Hello world!",
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub content: String,
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TargetType {
    User,
    Group,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Report {
    title: String,
    fields: Vec<ReportField>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReportField {
    key: String,
    value: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageIn {
    pub author_id: String, // "1234",
    pub target_id: String, // "4321",
    pub target_type: TargetType,
    pub message: Option<String>, // "Hello world!",
    pub report: Option<Report>,
    pub attachments: Vec<Attachment>,
}
