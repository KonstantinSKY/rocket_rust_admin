use  std::env;
use sea_orm::{DatabaseConnection, DbErr};
use rocket::fairing::AdHoc;

pub async fn init_database() -> Result<DatabaseConnection, DbErr> {
    let db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let host = env::var("POSTGRES_HOST").expect("POSTGRES_DB must be set");
    let port  = env::var("POSTGRES_PORT").expect("POSTGRES_DB must be set");
    let username  = env::var("POSTGRES_USERNAME").expect("POSTGRES_DB must be set");

    let database_url = format!("postgresql://{username}:{password}@{host}:{port}/{db}");
    println!("Data Base URL: {}", database_url);

    sea_orm::Database::connect(&database_url).await
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database Stage", |rocket| async {
        match init_database().await {
            Ok(database) => rocket.manage(database),
            Err(e) => {
                println!("Database connection failed: {}", e);
                rocket
            },
        }
    })
}
