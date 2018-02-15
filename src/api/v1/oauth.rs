//! OAuth module.

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use rocket_contrib::Json;
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::http::Status;
use uuid::Uuid;

use error::*;
use compress::CompressedJson;
use db::{self, CONNECTION_POOL};

/// Refresh token response structure.
#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    refresh_token: RefreshToken,
    access_token: AccessTokenResponse,
}

/// Refresh token information structure.
#[derive(Debug, Serialize)]
pub struct RefreshToken {
    #[serde(serialize_with = "hex_str")]
    token: [u8; 16],
    expiration: i64,
}

/// Token refresh request credentials.
#[derive(Debug, Deserialize)]
pub struct RefreshCredentials {
    username: String,
    password: String,
}

/// OAuth application request guard.
#[derive(Debug, Clone, Copy)]
pub struct Application {
    /// Application ID.
    id: Uuid,
    /// Number of requests left for the rest of the hour.
    requests_left: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for Application {
    type Error = &'static str;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        if let (Some(Ok(app_id)), Some(Ok(timestamp)), Some(signature)) = (
            request
                .headers()
                .get("X-App-Id")
                .next()
                .map(|str_id| str_id.parse::<u64>()),
            request
                .headers()
                .get("X-Timestamp")
                .next()
                .map(|str_time| str_time.parse::<i64>()),
            request.headers().get("X-Signature").next(),
        ) {
            let date_time = DateTime::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
            if date_time > Utc::now() - Duration::minutes(5)
                && date_time <= Utc::now() + Duration::seconds(10)
            {
                if let Ok(db_con) = CONNECTION_POOL.get() {
                    // Valid date time, check request.
                    match db::oauth::get_application(&db_con, app_id) {
                        Ok(Some(app)) => {
                            // It's ok, continue validating.
                            if let Ok(last_hour_count) =
                                db::cache::oauth::get_request_count(app.id())
                            {
                                if last_hour_count < app.hourly_limit() {
                                    // Everything ok, return structure.

                                    if let Err(_) = db::cache::oauth::add_request(app.id()) {
                                        // TODO log error.
                                        Outcome::Failure((
                                            Status::InternalServerError,
                                            "Unknown error",
                                        ))
                                    } else {
                                        Outcome::Success(Application {
                                            id: app.id(),
                                            requests_left: app.hourly_limit() - last_hour_count - 1,
                                        })
                                    }
                                } else {
                                    // Failure: too many requests.
                                    Outcome::Failure((
                                        Status::TooManyRequests,
                                        "Hourly request limit reached",
                                    ))
                                }
                            } else {
                                // TODO log error.
                                Outcome::Failure((Status::InternalServerError, "Unknown error"))
                            }
                        }
                        Ok(None) => {
                            // Failure: Invalid APP ID.
                            Outcome::Failure((Status::Unauthorized, "Invalid application ID"))
                        }
                        Err(_) => {
                            // TODO log error.
                            Outcome::Failure((Status::InternalServerError, "Unknown error"))
                        }
                    }
                } else {
                    // TODO log error.
                    Outcome::Failure((Status::InternalServerError, "Unknown error"))
                }
            } else {
                // Failure: Invalid date time.
                Outcome::Failure((Status::BadRequest, "Invalid timestamp"))
            }
        } else {
            // Failure: Invalid request.
            Outcome::Failure((
                Status::BadRequest,
                "Valid X-App-Id, X-Timestamp or X-Signature headers not found",
            ))
        }
    }
}

/// Authenticate user with username and password.
#[post("/refresh_token", data = "<credentials>")]
pub fn refresh_token(
    application: Application,
    credentials: Json<RefreshCredentials>,
) -> Result<CompressedJson<RefreshResponse>> {
    unimplemented!()
}

/// Access token information structure.
#[derive(Debug, Serialize)]
pub struct AccessToken {
    scope: Vec<Scope>,
    expiration: i64,
}

/// Get access token.
#[get("/access_token")]
pub fn access_token() -> Result<CompressedJson<AccessToken>> {
    unimplemented!()
}
