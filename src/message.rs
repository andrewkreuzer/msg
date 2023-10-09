use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Option<uuid::Uuid>,
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
    pub timestamp: Option<String>,
}

pub trait MessageService {
    fn send(&self, message: Message) -> anyhow::Result<Message>;
    fn recieve(&self, message: Message) -> anyhow::Result<Message>;
    fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()>;
}
