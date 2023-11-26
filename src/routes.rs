use axum::{
    extract::{State, TypedHeader},
    headers::Cookie,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tokio::fs::read;
use crate::structs::{Config, AuthRequests, UserStatus};

use std::sync::Arc;

pub async fn root(config: State<Arc<Config>>) -> (StatusCode, String) {
    let mut s = unsafe {
        String::from_utf8_unchecked(match (read(format!("{}/partial/index.html.top", config.static_path))).await {
            Ok(s) => s,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "oops".to_string())
        })
    };

    // if let Some(token_value)

    s.push_str(&format!(r#"
<a href="{}">git</a> |
<a href="settings.html">settings</a> |
<button onclick="logout()">Log Out</button>
        "#,
        config.git_url,
    ));


    s.push_str( unsafe {
        String::from_utf8_unchecked(match read(format!("{}/partial/index.html.bottom", config.static_path)).await {
            Ok(s) => s,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "oops".to_string())
        }).as_str()
    });


    (StatusCode::OK, s)
}

pub async fn auth_handler(
    config: State<Arc<Config>>,
    _cookie: TypedHeader<Cookie>,
    payload: Option<String>, // TODO: accept formdata only?
) -> Response {
    if let Some(mut data) = payload {
        if data.len() <= 1 {
            return StatusCode::BAD_REQUEST.into_response()
        }

        match AuthRequests::from(data.remove(0)) {
            AuthRequests::RequestSalt => {
                let values = sqlx::query!(
                    "SELECT salt, status FROM user WHERE username = ?;",
                    data,
                )
                .fetch_optional(&config.db)
                .await;

                if let Ok(Some(record)) = values {
                    if UserStatus::from_int_unchecked(record.status.unsigned_abs() as u8) != UserStatus::None {
                        record.salt.into_response()
                    } else {
                        StatusCode::FORBIDDEN.into_response()
                    }
                } else {
                    StatusCode::FORBIDDEN.into_response()
                }
            }
            AuthRequests::GenerateToken => {
                todo!()
            }
            AuthRequests::Register => {
                todo!()
            }
            AuthRequests::Logout => {
                todo!()
            }
            AuthRequests::None => StatusCode::BAD_REQUEST.into_response()
        }
    } else {
        StatusCode::BAD_REQUEST.into_response()
    }
}