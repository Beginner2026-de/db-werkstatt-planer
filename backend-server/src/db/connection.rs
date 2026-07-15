use surrealdb::engine::local::Db;
use surrealdb::Surreal;

type DbState = Surreal<Db>;
// 1. Initialisiere die eingebettete SurrealDB im lokalen Ordner "meine_db"
// Es wird kein externer Server benötigt und kein Login per .signin()!
pub async fn init_db() -> DbState {
    
    let db = Surreal::new::<surrealdb::engine::local::RocksDb>("meine_db")
        .await
        .expect("Fehler beim Starten der Embedded-Datenbank");

    // 2. Namespace und DB festlegen
    db.use_ns("main").use_db("main")
        .await
        .expect("Fehler beim Auswählen von Namespace/DB");
    db
}