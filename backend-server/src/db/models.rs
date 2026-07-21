use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb::types::SurrealValue;

type DbState = Surreal<Db>;

// 1. Datenstruktur definieren
#[derive(Debug, Serialize, Deserialize, SurrealValue)]
pub struct User {
    pub name: String,
    pub password: String,
}

// 2. Funktion zum Erstellen / Einfügen eines Eintrags
pub async fn create_user_admin(db: &DbState, name: String, password: String) -> Result<(), surrealdb::Error> {
    let user_admin = json!({
        "name": name,
        "password": password,
    });

    // .create("tabelle") erstellt die Tabelle automatisch, falls sie noch nicht existiert
    let created: Option<User> = db
        .create("user_admin")
        .content(user_admin)
        .await?;

    println!("User erfolgreich angelegt: {:?}", created);
    Ok(())
}

// 3. Funktion zum Abfragen aller Einträge aus einer Tabelle
pub async fn get_all_users_admin(db: &DbState) -> Result<Vec<User>, surrealdb::Error> {
    // .select("tabelle") ruft alle Datensätze einer Tabelle ab
    let users: Vec<User> = db.select("user_admin").await?;
    Ok(users)
}
