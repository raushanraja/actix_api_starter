use actix_cors::Cors;
use actix_web::{App, HttpServer};
use api_starter::{api::v1::endpoints::root::root, config::configure};
use uuid::Uuid;

async fn serve() {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let server_id = Uuid::new_v4();

    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new().app_data(server_id).wrap(cors).service(root)
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect("Error starting")
    .run()
    .await;
}

#[actix_web::main]
async fn main() {
    configure();
    serve().await;
}
