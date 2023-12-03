use crate::{
    error::ColloError,
    structs::{AuthRequests, Config, GenerateTokenReq, UserStatus},
};

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use cookie::SameSite;
use sha2::{Digest, Sha512};
use time::{Duration, OffsetDateTime};
use tower_cookies::{Cookie, Cookies};
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

pub async fn auth_page(
    config: State<Arc<Config>>,
) -> Result<Html<String>, ColloError> {
    Ok(Html(String::from_utf8_lossy(
        &read(format!("{}/auth.html", &config.static_path)).await?).to_string()))

} 

pub async fn auth_handler(
    config: State<Arc<Config>>,
    jar: Cookies,
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
                let json_data = serde_json::from_str::<GenerateTokenReq>(&data)?;
                
                let query = sqlx::query!(
                    "SELECT password, id, status FROM user WHERE username = ?;",
                    json_data.username,
                )
                .fetch_optional(&config.db)
                .await?;

                if let Some(record) = query {
                    let (pswd, id, status): (Vec<u8>, _, _) = 
                            (record.password, record.id, record.status);

                    if UserStatus::from_int_unchecked(status.unsigned_abs() as u8) == UserStatus::None {
                        return Ok(StatusCode::FORBIDDEN.into_response());
                    }

                    let pswd = config.hex_as_string(pswd);
                    let hashed_sent_pswd = format!("{:x}", 
                                Sha512::digest(json_data.password.as_bytes()));

                    if pswd != hashed_sent_pswd {
                        return Ok(StatusCode::FORBIDDEN.into_response());
                    } 

                    let token = config.generate_token().await;

                    sqlx::query!(
                        "INSERT INTO token VALUES (?, ?, unixepoch())",
                        token,
                        id,
                    )
                    .execute(&config.db)
                    .await?;

                    jar.add(Cookie::build(("token", token.clone()))
                            .domain(config.address.to_string())
                            .path("/")
                            .secure(true)
                            .http_only(true)
                            .expires(OffsetDateTime::now_utc() + Duration::days(30))
                            .same_site(SameSite::Strict)
                            .into()
                    );

                    Ok(StatusCode::NO_CONTENT.into_response())
                } else {
                    Ok(StatusCode::FORBIDDEN.into_response())
                } 
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
