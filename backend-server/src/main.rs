use surrealdb::Surreal;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, CorsOptions
};
use routes::auth::{login, register};
mod models;
mod routes;

<<<<<<< HEAD
// Typ-Alias für saubereren Code in den Routen
type DbState = Surreal<Db>;


// Wir definieren ein Struct für die Antwort an Tauri
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct LoginResponse {
    message: String,
    success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct LoginRequest {
    pub login_username: String,
    pub login_password: String,
}

#[post("/login",format = "json", data = "<login_data>")]
async fn login(
    db: &State<DbState>,
    login_data: Json<LoginRequest>
    ) -> Result<Json<LoginResponse>, String> {
    rocket::info!("Hallo");
    let data = login_data.into_inner();

    rocket::info!("Name {}, Passwort {}", data.login_username, data.login_password);

    Ok(Json(LoginResponse {
        message: "Login successful!".into(),
        success: true,
    }))
}

#[post("/register")]
async fn register() -> &'static str {
    "Registration successful!"
}
=======
>>>>>>> 574a080 (split up code)

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