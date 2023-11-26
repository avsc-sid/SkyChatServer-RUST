use rand::{Rng, distributions::Alphanumeric};
use sqlx::sqlite::SqlitePool;

use std::fmt;
use std::net::SocketAddr;

#[macro_export]
macro_rules! dotenv_get {
    ($var:expr) => {
        dotenv::var($var).expect(concat!("Missing ", $var, " environmental variable"))
    };
    ($var:expr, $parse_as:ty) => {
        dotenv::var($var)
            .expect(concat!("Missing ", $var, " environmental variable"))
            .parse::<$parse_as>()
            .expect(concat!($var, " must be valid ", stringify!($parse_as)))
    };
}

#[macro_export]
macro_rules! create_enums {
    ($($name:ident: [$($count:literal => $variants:ident),*]),+) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum $name {
                $(
                    $variants = $count
                ),*
                ,None = 127
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    match *self {
                        $(
                            $name::$variants => write!(f, stringify!($variants))
                        ),*
                        ,$name::None => write!(f, "None"),
                    }
                }
            }

            impl std::str::FromStr for $name {
                type Err = ();

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    if let Some(c) = s.chars().next() {
                        return Ok(Self::from(c));
                    }
                    Err(())
                }
            }

            impl From<char> for $name {
                fn from(value: char) -> Self {
                    match value as i8 - '0' as i8 {
                        $(
                            $count => Self::$variants,
                        )*
                        _ => Self::None
                    }
                }
            }

            impl $name {
                #[allow(dead_code)]
                pub fn from_int_unchecked(num: u8) -> Self {
                    unsafe { std::mem::transmute(num) }
                }
            }
        )+
    }
}

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
    pub run: SocketAddr,
    pub git_url: String,
}

impl Config {
    pub async fn init() -> Self {
        Self {
            db: SqlitePool::connect(dotenv_get!("DATABASE_URL").as_str()).await.unwrap(),
            static_path: dotenv_get!("STATIC_PATH"),
            private_path: dotenv_get!("PRIVATE_PATH"),
            token_len: dotenv_get!("TOKEN_LEN", usize),
            chat_channel_capacity: dotenv_get!("CHAT_CHANNEL_CAPACITY", usize),
            allow_user_creation: dotenv_get!("ALLOW_USER_CREATION", bool),
            run: dotenv_get!("RUN").parse().expect("Does not contain valid socket address!"),
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
