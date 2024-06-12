use sea_orm::{DatabaseConnection, DbErr};
use rocket::fairing::AdHoc;

pub async fn init_database() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
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
