use rocket::http::{Header,Method} ;
use rocket::{Request, Response, Data};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{get, post, routes, State, options};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;


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
            kind: Kind::Request 
        }
    }

async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        // Wenn es OPTIONS war, zwingend mit 200 OK antworten
        if request.method() == Method::Options {
            response.set_status(rocket::http::Status::Ok);
            response.set_sized_body(0, std::io::Cursor::new(""));
        }
    }
}
#[options("/<path..>")]
fn all_options(path: std::path::PathBuf) -> &'static str {
    ""
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct LoginRequest {
    login_username: String,
    login_password: String,
}

#[post("/login", data = "<login_data>")]
async fn login(
    db: &State<DbState>,
    login_data: Json<LoginRequest>
    ) -> Result<Json<LoginResponse>, String> {

    let username = &login_data.login_username;
    let paswort = &login_data.login_password;
    rocket::info!("Name {}, Passwort {}", username, paswort);

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
        .attach(CORS)
        .manage(db) // Hier wird die DB für alle Routen registriert
        .mount("/", routes![all_options]) // <-- Hier mitsenden!
        .mount("/api/auth", routes![login, register])
        .launch()
        .await?;

    Ok(())
}