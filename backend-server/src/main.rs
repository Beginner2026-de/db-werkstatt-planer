use surrealdb::Surreal;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, CorsOptions
};
use routes::auth::{login, register};
mod models;
mod routes;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // 1. Initialisiere die eingebettete SurrealDB im lokalen Ordner "meine_db"
    // Es wird kein externer Server benötigt und kein Login per .signin()!
    let db = Surreal::new::<surrealdb::engine::local::RocksDb>("meine_db")
        .await
        .expect("Fehler beim Starten der Embedded-Datenbank");

    // 2. Namespace und DB festlegen
    db.use_ns("main").use_db("main")
        .await
        .expect("Fehler beim Auswählen von Namespace/DB");

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