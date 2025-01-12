use crate::errors::CustomError;
use axum::{Extension, Json};
use db::User;

pub async fn loader(Extension(pool): Extension<db::Pool>) -> Result<Json<Vec<User>>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    Ok(Json(users))
}