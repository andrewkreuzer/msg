use axum::{
    extract::{
        ws::{self, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{net::SocketAddr, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    message::{Message, MessageService},
    state,
    user::User,
};

pub struct Server {
    _addr: Option<String>,
}

impl Server {
    pub async fn new() -> Self {
        Self { _addr: None }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "msg=trace".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

        let state = Arc::new(Mutex::new(state::State::new()));
        let app = Router::new()
            .route("/", get(index))
            .route("/connect", get(connect))
            .route("/users", get(users))
            .with_state(state);

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        info!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}

impl MessageService for state::State {
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

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../../chat.html"))
}

async fn connect(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<state::State>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<Mutex<state::State>>) {
    let (mut sender, mut receiver) = stream.split();

    let mut user: Option<User> = None;
    let mut recv_task: Option<JoinHandle<()>> = None;
    while let Some(Ok(message)) = receiver.next().await {
        if let ws::Message::Text(name) = message {
            let mut state = state.lock().await;
            let state_user = state.get_user(&name).await;
            match state_user {
                Ok(_) => {
                    debug!("Username already taken");
                    let _ = sender
                        .send(ws::Message::Text(String::from("Username already taken.")))
                        .await;
                    return;
                }
                Err(_) => {
                    debug!("Username available adding user");
                    let (u, rt) = User::new(name, sender).await;
                    let _ = state.add_user(&u).await;
                    user = Some(u);
                    recv_task = Some(rt);
                    break;
                }
            };
        }
    }

    let user = if let Some(user) = user {
        debug!("User made keeping connection");
        user
    } else {
        debug!("User emtpy closing connection");
        return;
    };

    let s = state.clone();
    let u = user.clone();
    tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            if let ws::Message::Text(message) = message {
                let user_msg: Vec<&str> = message.split("::").collect();
                let to = user_msg[0];
                let msg = user_msg[1];
                if to == "all" {
                    s.lock().await.send_all(msg).await;
                    continue;
                }
                s.lock()
                    .await
                    .get_user(to)
                    .await
                    .unwrap()
                    .send(msg.to_string())
                    .await;

                u.send(format!("{to}: {msg}")).await;
            }
        }
    });

    let msg = format!("{} has joined the chat", user.username);
    state.lock().await.send_all(&msg).await;

    tokio::select! {
        _ = recv_task.unwrap() => (),
    };

    let msg = format!("{} has left the chat", user.username);
    state.lock().await.send_all(&msg).await;
    state.lock().await.users.remove(&user.username);
}

async fn users(State(state): State<Arc<Mutex<state::State>>>) -> String {
    let state = state.lock().await;
    state.get_users()
}
