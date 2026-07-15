use rocket::post;
use rocket::serde::json::Json;
use crate::models::users::{LoginRequest, LoginResponse};

#[post("/login",format = "json", data = "<login_data>")]
pub async fn login(
    //db: &rocket::State<crate::DbState>,
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
pub async fn register() -> &'static str {
    "Registration successful!"
}