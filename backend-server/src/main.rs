#[macro_use] extern crate rocket;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

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
    "Login successful!"
}

#[get("/register")]
fn register() -> &'static str {
    "Registration successful!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/api/auth", routes![login, register])
}