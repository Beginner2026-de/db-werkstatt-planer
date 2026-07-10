use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{get, post, routes, State};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
// Deine Datenstruktur. 
// Wichtig in SurrealDB 3.x+: Eigene Structs müssen zusätzlich `SurrealValue` implementieren!
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct LoginRequest {
    login_username: String,
    login_password: String,
}
// Wir definieren ein Struct für die Antwort an Tauri
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct LoginResponse {
    message: String,
    success: bool,
}

// Typ-Alias für saubereren Code in den Routen
type DbState = Surreal<Db>;
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*")); // Erlaubt Zugriff von überall
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}


#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[post("/login", data = "<login_data>")]
async fn login(
    db: &State<DbState>,
    login_data: Json<LoginRequest>
    ) -> Result<Json<LoginResponse>, String> {
    Ok(Json(LoginResponse {
        message: "Login successful!".into(),
        success: true,
    }))
}

#[post("/register")]
async fn register() -> &'static str {
    "Registration successful!"
}

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

    // 3. Rocket starten und die DB als State übergeben
    let _ = rocket::build()
        .manage(db) // Hier wird die DB für alle Routen registriert
        .mount("/api/auth", routes![login, register])
        .launch()
        .await?;

    Ok(())
}