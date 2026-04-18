mod db;
mod errors;
mod handlers;
mod middleware;
mod models;

use axum::{
    middleware as axum_middleware,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// ─── AppState ─────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct AppState {
    pub db: db::Db,
    pub jwt_secret: String,
}

// ─── OpenAPI Definition ───────────────────────────────────────────────────────

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Axum API",
        version = "1.0.0",
        description = "Production-ready Axum REST API with JWT auth",
        contact(name = "Dev Team", email = "dev@example.com"),
    ),
    paths(
        // Auth
        handlers::user::register,
        handlers::user::login,
        // Users (protected)
        handlers::user::list_users,
        handlers::user::get_user,
        handlers::user::update_user,
        handlers::user::delete_user,
    ),
    components(
        schemas(
            // Request bodies
            models::user::CreateUserRequest,
            models::user::UpdateUserRequest,
            models::user::LoginRequest,
            // Responses
            models::user::UserResponse,
            models::user::LoginResponse,
            models::user::UsersListResponse,
            // Error
            errors::ErrorResponse,
        )
    ),
    tags(
        (name = "Auth",  description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints (JWT required)"),
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

// Add bearer auth scheme to every protected route
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = &mut openapi.components {
            use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

// ─── Router ───────────────────────────────────────────────────────────────────

fn protected_user_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::user::list_users))
        .route("/{id}", get(handlers::user::get_user))
        .route("/{id}", put(handlers::user::update_user))
        .route("/{id}", delete(handlers::user::delete_user))
        .route_layer(axum_middleware::from_fn_with_state(
            state,
            middleware::auth::require_auth,
        ))
}

fn api_router(state: AppState) -> Router<AppState> {
    Router::new()
        // Public auth routes
        .route("/auth/register", post(handlers::user::register))
        .route("/auth/login", post(handlers::user::login))
        // Protected user routes
        .nest("/users", protected_user_routes(state))
}

fn build_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // ── Swagger UI at /docs ──
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // ── API v1 ──
        .nest("/api/v1", api_router(state.clone()))
        .with_state(state)
        .layer(cors)
}

// ─── main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    // Load .env
    dotenvy::dotenv().ok();

    // Tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_utoipa=debug,tower_http=debug".into()),
        )
        .init();

    // Config from env
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret   = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let host         = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port         = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".into());

    // Database pool
    let db = db::create_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations").run(&db).await.expect("Migration failed");

    let state = AppState { db, jwt_secret };
    let app   = build_app(state);

    let addr = format!("{}:{}", host, port);
    tracing::info!("🚀 Server listening on http://{}", addr);
    tracing::info!("📖 Swagger UI at  http://{}/docs", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
