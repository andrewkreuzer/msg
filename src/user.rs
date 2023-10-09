use axum::extract::ws::{Message, WebSocket};
use futures::sink::SinkExt;
use futures::stream::SplitSink;
use tokio::{
    sync::mpsc::{self, Sender},
    task::JoinHandle,
};
use tracing::info;

#[derive(Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub tx: Sender<String>,
}

impl User {
    pub async fn new(
        username: String,
        mut sender: SplitSink<WebSocket, Message>,
    ) -> (Self, JoinHandle<()>) {
        let (tx, mut rx) = mpsc::channel(100);

        let id = uuid::Uuid::new_v4();
        let recv_task = tokio::spawn(async move {
            loop {
                let result = rx.recv().await;
                match result {
                    Some(message) => {
                        if let Err(e) = sender.send(Message::Text(message)).await {
                            eprintln!("Failed to send message for user {}: {:?}", id, e);
                        }
                    }
                    None => {
                        info!("Channel closed");
                        break;
                    }
                }
            }
        });

        (Self { id, username, tx }, recv_task)
    }

    pub async fn send(&self, message: String) {
        if let Err(e) = self.tx.send(message).await {
            eprintln!("Failed to send message for user {}: {:?}", self.id, e);
        }
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            username: self.username.clone(),
            tx: self.tx.clone(),
        }
    }
}
