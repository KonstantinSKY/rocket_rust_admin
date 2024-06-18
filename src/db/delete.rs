// use rocket::State;
// use sea_orm::{entity::prelude::*, DatabaseConnection, DbErr};

// pub async fn delete<E>(db: &State<DatabaseConnection>, id: i32) -> Result<u64, DbErr>
// where
//     E: EntityTrait,
//     E::PrimaryKey: From<i32> + PrimaryKeyTrait,
//     <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<<E as EntityTrait>::PrimaryKey>,
// {
//     let primary_key = E::PrimaryKey::from(id);
//     let result = E::delete_by_id(primary_key).exec(db.inner()).await;

//     match result {
//         Ok(delete_result) => Ok(delete_result.rows_affected),
//         Err(err) => Err(err),
//     }
// }