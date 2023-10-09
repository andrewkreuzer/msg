use crate::message::{Message, MessageService};

struct Client {
    _state: String,
}

impl MessageService for Client {
    fn send(&self, message: Message) -> anyhow::Result<Message> {
        println!("Sending message: {:?}", message);
        Ok(message)
    }
    fn recieve(&self, message: Message) -> anyhow::Result<Message> {
        println!("Receiving message for: {:?}", message);
        Ok(message)
    }
    fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()> {
        println!("Deleting message: {:?}", id);
        Ok(())
    }
}
