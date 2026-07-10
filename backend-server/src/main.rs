use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{get, post, routes, launch, State};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};

// Deine Datenstruktur. 
// Wichtig in SurrealDB 3.x+: Eigene Structs müssen zusätzlich `SurrealValue` implementieren!
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    completed: bool,
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
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/login")]
fn login() -> &'static str {
    println!("Login function called, db_main returned: {:?}", anser);
    return "Login successful!"
}

#[get("/register")]
fn register() -> &'static str {
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