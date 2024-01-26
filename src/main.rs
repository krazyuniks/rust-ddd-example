mod application;
mod db;
use sqlx::Transaction;

use application::entities::user::{Password, User, UserName, UserRoles, UserTrait};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting User test");

    let db = db::connect().await;

    /*
    sqlx::query!("TRUNCATE TABLE users")
        .execute(&db)
        .await
        .expect("truncate table users");
    */

    let mut tx: Transaction<'static, sqlx::Postgres> = db.begin().await.unwrap();

    let user = User::new(
        UserName::try_from("John".to_string()).unwrap(),
        Password::try_from("eightmin".to_string()).unwrap(),
        UserRoles::try_from(vec!["ADMIN".to_string(), "USER".to_string()]).unwrap(),
    );
    //println!("\nuser {:#?}\n", user.id.value());

    match user.save(&mut tx).await {
        Ok(_) => println!("user saved in tx"),
        Err(e) => panic!("user error: {}", e),
    }

    match tx.commit().await {
        Ok(_) => println!("tx commited"),
        Err(e) => println!("tx error: {}", e),
    };

    Ok(())
}
