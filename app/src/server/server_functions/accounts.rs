use dioxus_fullstack::prelude::*;

#[server(CreateAccount, "/api")]
pub async fn create_account(username: String, password: String) -> Result<(), ServerFnError> {
    use crate::server::{auth, database};

    let hashed_password = auth::hash_password(&password).unwrap();
    sqlx::query!(
        "INSERT INTO accounts (username, password) VALUES ($1, $2)",
        username,
        hashed_password,
    )
    .execute(database::POOL.get().unwrap())
    .await?;

    Ok(())
}

#[server(VerifyAccount, "/api")]
pub async fn verify_account(username: String, password: String) -> Result<bool, ServerFnError> {
    use crate::server::{auth, database};

    let record = sqlx::query!(
        "SELECT password FROM accounts WHERE username = $1",
        username
    )
    .fetch_one(database::POOL.get().unwrap())
    .await?;

    Ok(auth::verify_password(&password, &record.password)?)
}