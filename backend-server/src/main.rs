use rocket_cors::{
    AllowedHeaders, AllowedOrigins, CorsOptions
};

mod routes;
use routes::auth::{login, register};

mod models;

mod db;
use db::connection::init_db;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = init_db().await;

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![
            rocket::http::Method::Get.into(),
            rocket::http::Method::Post.into(),
            rocket::http::Method::Options.into(),
        ]
        .into_iter()
        .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS-Konfiguration ungültig");

    // 3. Rocket starten und die DB als State übergeben
    let _ = rocket::build()
        .attach(cors)
        .manage(db) // Hier wird die DB für alle Routen registriert
        .mount("/api/auth", rocket::routes![login, register])
        .launch()
        .await?;

    Ok(())
}