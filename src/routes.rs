use crate::{
    error::ColloError,
    structs::{AuthRequests, Config, UserStatus},
};
use axum::{
    extract::{State, TypedHeader},
    headers::Cookie,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use tokio::fs::read;

use std::sync::Arc;

pub async fn root(config: State<Arc<Config>>) -> Result<Html<String>, ColloError> {
    let mut s: String =
        String::from_utf8_lossy(&read(format!("{}/partial/index.html.top", &config.static_path)).await?).to_string();

    // if let Some(token_value)

    s.push_str(&format!(
        r#"
<a href="{}">git</a> |
<a href="settings.html">settings</a> |
<button onclick="logout()">Log Out</button>
        "#,
        config.git_url,
    ));

    s.push_str(
        &String::from_utf8_lossy(
            &read(format!("{}/partial/index.html.bottom", &config.static_path)).await?,
        )
    );

    Ok(Html(s))
}

pub async fn auth_handler(
    config: State<Arc<Config>>,
    _cookie: TypedHeader<Cookie>,
    payload: Option<String>, // TODO: accept formdata only?
) -> Result<Response, ColloError> {
    if let Some(mut data) = payload {
        if data.len() <= 1 {
            return Ok(StatusCode::BAD_REQUEST.into_response());
        }

        match AuthRequests::from(data.remove(0)) {
            AuthRequests::RequestSalt => {
                let values =
                    sqlx::query!("SELECT salt, status FROM user WHERE username = ?;", data,)
                        .fetch_optional(&config.db)
                        .await?;

                if let Some(record) = values {
                    if UserStatus::from_int_unchecked(record.status.unsigned_abs() as u8)
                        != UserStatus::None
                    {
                        Ok(record.salt.into_response())
                    } else {
                        Ok(StatusCode::FORBIDDEN.into_response())
                    }
                } else {
                    Ok(StatusCode::FORBIDDEN.into_response())
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
            AuthRequests::None => Ok(StatusCode::BAD_REQUEST.into_response()),
        }
    } else {
        Ok(StatusCode::BAD_REQUEST.into_response())
    }
}
