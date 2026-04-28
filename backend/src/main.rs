use std::net::SocketAddr;

use axum::{
    Router,
    extract::ConnectInfo,
    http::{HeaderMap, Method},
    routing::get,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, TraceLayer},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{config::state::AppState};

pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod queue;
pub mod services;
pub mod utils;
pub mod workers;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

fn get_client_ip(headers: &HeaderMap, connect_info: Option<SocketAddr>) -> Option<String> {
    if let Some(forwared) = headers.get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwared.to_str() {
            return forwarded_str
                .split(",")
                .next()
                .map(|s| s.trim().to_string());
        }
    }

    connect_info.map(|addr| addr.ip().to_string())
}

async fn your_ip(ConnectInfo(addr): ConnectInfo<SocketAddr>, headers: HeaderMap) -> String {
    let ip = get_client_ip(&headers, Some(addr));
    println!("Client IP: {:?}", ip);
    ip.unwrap()
}

pub async fn run() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(Any);

    let state = AppState::new().await;
    let _ = workers::start_all(state.redis_publisher.clone(), state.pool.clone()).await;

    let router: Router = Router::new()
        .nest(
            "/api",
            Router::new()
            .nest("/auth", handlers::auth::router())
            .merge( handlers::link::router(state.clone()))
        )
        .merge(handlers::redirect::router())
        .route("/test", get(your_ip))
        .with_state(state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true)),
        )
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()));

    let port = 3080;
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Port is already been used!");

    println!("Listening on :{addr}");

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Error serving application");
}

#[tokio::main]
async fn main() {
    // loading the .env to project
    dotenvy::dotenv().ok();
    utils::logger::init_tracing();

    let r = utils::email_templates::welcome_email("Good man");
      match utils::email::send("xeyave3913@hacknapp.com", r.subject, r.text_body, r.html_body).await {
       Ok(_) =>  println!("Successfully send mail!"),
       Err(e) => println!("{:#?}",e.to_string())
   };

    run().await;
}

/*   add test later
   let example = "facebookexternalhit/1.1 (+http://www.facebook.com/externalhit_uatext.php)";
   println!("{:#?}", utils::user_agent::parse_user_agent(example));

       let db = utils::geo::init_geo_db();
       println!("{:?}", utils::geo::lookup(&db, "192.193.212.221").await);

           match email::send("rishavsingh076@gmail.com", "I am feeling good", "Hello nice to meet you".to_string()).await {
       Ok(_) =>  println!("Successfully send mail!"),
       Err(e) => println!("{:#?}",e.to_string())
   };


*/
