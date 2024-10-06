use ::unkey::{models::VerifyKeyRequest, Client as UnkeyClient};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct UnkeyApiId(String);

impl From<UnkeyApiId> for String {
    fn from(api_id: UnkeyApiId) -> Self {
        api_id.0
    }
}

#[derive(Clone)]
struct AppState {
    unkey_client: UnkeyClient,
    unkey_api_id: UnkeyApiId,
}

impl FromRef<AppState> for UnkeyClient {
    fn from_ref(state: &AppState) -> Self {
        state.unkey_client.clone()
    }
}

impl FromRef<AppState> for UnkeyApiId {
    fn from_ref(state: &AppState) -> Self {
        state.unkey_api_id.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let unkey_root_key = env::var("UNKEY_ROOT_KEY").unwrap_or_default();
    let unkey_api_id = UnkeyApiId(env::var("UNKEY_API_ID").unwrap_or_default());
    let unkey_client = UnkeyClient::new(&unkey_root_key);

    // Create the application state with the Unkey client and API ID
    let app_state = AppState {
        unkey_client,
        unkey_api_id,
    };

    // Build the Axum application with routes for public and protected handlers
    let app = Router::new()
        .route("/public", get(public_handler))
        .route("/protected", get(protected_handler))
        .with_state(app_state);

    let addr = format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or_else(|_| "3000".to_string())
    );
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Handles requests for the public route. The user session is optional.
// If a user is logged in, a personalized welcome message is returned.
// If no user is logged in, a general welcome message for guests is provided.
async fn public_handler(user: Option<User>) -> impl IntoResponse {
    match user {
        Some(u) => format!(
            "Welcome back, {}! You’re successfully logged in.\nFeel free to explore the `/protected` route!",
            u.username
        ),
        None => "Hello, Guest! It looks like you’re not logged in yet. No worries—this route is open for everyone.".to_string(),
    }
}

// Handles requests for the protected route. A valid user session is required.
// If the user is not authenticated, a 401 UNAUTHORIZED status is returned.
// Upon successful authentication, the user's information is returned.
async fn protected_handler(user: User) -> impl IntoResponse {
    format!("Welcome to the protected area :)\nHere's your info:\n{user:?}")
}

// User struct representing the data returned for authenticated users
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    key_id: String,
}

// Implementation of FromRequestParts trait for extracting a User from request parts
#[async_trait]
impl<S> FromRequestParts<S> for User
where
    UnkeyClient: FromRef<S>,
    UnkeyApiId: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let client = UnkeyClient::from_ref(state);
        let api_id: String = UnkeyApiId::from_ref(state).into();
        let req = VerifyKeyRequest::new(bearer.token(), &api_id);

        // Verify the key and return user data if valid
        match client.verify_key(req).await {
            Ok(res) if res.valid => {
                // Now you can load the user data from your DB
                Ok(User {
                    id: "yAXutuxsKe".into(),
                    username: "robinsonkayla".into(),
                    key_id: res.key_id.unwrap_or("No key in Unkey response".into()),
                })
            }
            _ => {
                eprintln!("Unauthorized access attempt");
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    }
}
