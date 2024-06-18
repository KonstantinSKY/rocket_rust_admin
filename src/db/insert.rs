use rocket::State;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, IntoActiveModel};
use sea_orm::EntityTrait; 


pub async fn insert<E, A>(db: &State<DatabaseConnection>, model: A) -> Result<E::Model, DbErr>
where
    E: EntityTrait,                                 // E is the entity
    A: ActiveModelTrait<Entity = E> + Send, 
    E::Model: IntoActiveModel<A>,                   // A is the ActiveModel associated with E
{
    let result = E::insert(model)
        .exec_with_returning(db.inner())         // For Postgres only
        .await;

        match result {
            Ok(inserted_model) => Ok(inserted_model),
            Err(err) => Err(err),
        }
}