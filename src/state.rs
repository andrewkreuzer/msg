use anyhow::{anyhow, Result};
use std::collections::HashMap;
use tracing::debug;

use crate::user::User;

#[derive(Debug)]
pub struct State {
    pub users: HashMap<String, User>,
}

impl State {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub async fn add_user(&mut self, user: &User) -> Result<(), anyhow::Error> {
        if self.users.contains_key(&user.username) {
            debug!("User already exists");
            return Err(anyhow!("User already exists"));
        }

        self.users.insert(user.username.clone(), user.clone());
        debug!("Added user");
        Ok(())
    }

    pub fn get_users(&self) -> String {
        self.users
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub async fn get_user(&self, username: &str) -> Result<&User, anyhow::Error> {
        if !self.users.contains_key(username) {
            return Err(anyhow!("User does not exist"));
        }
        Ok(&self.users[username])
    }

    pub async fn send_all(&self, message: &str) {
        for user in self.users.values() {
            user.send(message.to_string()).await;
        }
    }
}
