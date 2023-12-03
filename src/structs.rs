use std::net::SocketAddr;

use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct GenerateTokenReq {
    pub username: String,
    pub password: String,
} 

#[derive(Debug, Clone)]
pub struct Config {
    pub db: SqlitePool,
    pub static_path: String,
    pub private_path: String,
    pub token_len: usize,
    pub token_expiry: u64,
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
            token_expiry: dotenv_get!("TOKEN_EXPIRY", u64),
            chat_channel_capacity: dotenv_get!("CHAT_CHANNEL_CAPACITY", usize),
            allow_user_creation: dotenv_get!("ALLOW_USER_CREATION", bool),
            address: dotenv_get!(ADDRESS_ENV, SocketAddr),
            git_url: dotenv_get!("GIT_URL"),
        }
    }

    pub async fn generate_token(&self) -> String {
        loop {
            let token: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(self.token_len)
                .map(char::from)
                .collect();

            if self.id_from_token(token.clone()).await == None {
                break token;
            } 
        } 
    }

    pub async fn id_from_token(&self, token: String) -> Option<i64> { 
        let query = sqlx::query!(
            "SELECT user_id AS value FROM token WHERE token = ?;",
            token,
        )
        .fetch_optional(&self.db)
        .await;

        if let Ok(Some(id)) = query {
            Some(id.value)
        } else {
            None
        } 
    } 

    pub fn _delete_old_tokens(&self) {
        // remove all expired tokens
        // if tokens are more than TOKEN_MAX, delete sufficient old tokens
        // can be used to prevent infinite id_from_token()

        todo!()
    } 

    fn nibble_to_hexchar(&self, b: u8) -> char {
        match b {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => unreachable!(), 
        } 
    } 

    pub fn hex_as_string(&self, pswd: Vec<u8>) -> String {
        let mut string = String::new();

        for c in pswd {
            string.push(self.nibble_to_hexchar((c & 0xf0) >> 4));
            string.push(self.nibble_to_hexchar(c & 0x0f));
        } 
        string
    } 
}
