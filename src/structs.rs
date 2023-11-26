use std::net::SocketAddr;

use rand::{distributions::Alphanumeric, Rng};
use sqlx::SqlitePool;

use crate::{create_enums, dotenv_get};

create_enums! {
    AuthRequests: [0 => RequestSalt, 1 => GenerateToken, 2 => Register, 3 => Logout],
    ChatRequests: [0 => RelayedMessage, 1 => LoadMoreMessages],
    AdminRequests: [0 => Update, 1 => Delete, 2 => Create],

    // admins have a negative status
    UserStatus: [
        0 => Leader,
        1 => Deputy,
        2 => Healer,
        3 => Mediator,
        4 => SeniorWarrior,
        5 => Warrior,
        6 => Apprentice,
        7 => Kit
    ]
}

#[derive(Debug, Clone)]
pub struct Config {
    pub db: SqlitePool,
    pub static_path: String,
    pub private_path: String,
    pub token_len: usize,
    pub chat_channel_capacity: usize,
    pub allow_user_creation: bool,
    pub address: SocketAddr,
    pub git_url: String,
}

impl Config {
    pub async fn init() -> Self {
        const ADDRESS_ENV: &str = if cfg!(debug_assertions) {
            "DEBUG_ADDRESS"
        } else {
            "PROD_ADDRESS"
        };
        Self {
            db: SqlitePool::connect(dotenv_get!("DATABASE_URL").as_str())
                .await
                .unwrap(),
            static_path: dotenv_get!("STATIC_PATH"),
            private_path: dotenv_get!("PRIVATE_PATH"),
            token_len: dotenv_get!("TOKEN_LEN", usize),
            chat_channel_capacity: dotenv_get!("CHAT_CHANNEL_CAPACITY", usize),
            allow_user_creation: dotenv_get!("ALLOW_USER_CREATION", bool),
            address: dotenv_get!(ADDRESS_ENV, SocketAddr),
            git_url: dotenv_get!("GIT_URL"),
        }
    }

    pub fn generate_token(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(self.token_len)
            .map(char::from)
            .collect()
    }
}
