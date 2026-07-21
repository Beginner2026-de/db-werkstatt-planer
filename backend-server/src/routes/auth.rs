use rocket::post;
use rocket::serde::json::Json;
use crate::models::users::{LoginRequest, LoginResponse};

use crate::db::models::{create_user_admin, get_all_users_admin};

#[post("/login", format = "json", data = "<login_data>")]
pub async fn login(
    db: &rocket::State<surrealdb::Surreal<surrealdb::engine::local::Db>>,
    login_data: Json<LoginRequest>
) -> Result<Json<LoginResponse>, String> {
    println!("DEBUG: reached login handler");
    let users = get_all_users_admin(db)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    rocket::info!("Users: {:?}", users);

    let LoginRequest {
        login_username,
        login_password,
    } = login_data.into_inner();

    rocket::info!("Name {}, Passwort {}", login_username, login_password);

    create_user_admin(db, login_username, login_password)
        .await
        .map_err(|e| format!("DB error: {}", e))?;

    let users = get_all_users_admin(db)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    rocket::info!("Users: {:?}", users);

    Ok(Json(LoginResponse {
        message: "Login successful!".into(),
        success: true,
    }))
}

#[post("/register")]
pub async fn register() -> &'static str {
    "Registration successful!"
}