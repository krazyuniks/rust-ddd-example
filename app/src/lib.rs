mod db;
mod entities;
use sqlx::error::BoxDynError;
use sqlx::Transaction;

use entities::user::{find_by_id, Password, User, UserId, UserName, UserRoles, UserTrait};

pub async fn execute() -> Result<Option<User>, Box<dyn std::error::Error>> {
    println!("Starting User test");

    let db = db::connect().await;

    let found = find_by_id(&db, "a154d3bc-a5f6-461d-8463-2cff24b19308").await;

    match found {
        Ok(user) => {
            println!("user found: {}", user.get_username());
        }
        Err(e) => {
            println!("user not found: {}", e);
        }
    }

    /*
    sqlx::query!("TRUNCATE TABLE users")
        .execute(&db)
        .await
        .expect("truncate table users failed");
    */

    let mut tx: Transaction<'static, sqlx::Postgres> = db.begin().await.unwrap();

    let user = User::new(
        UserId::new(),
        UserName::try_from("John2".to_string()).expect("Invalid username"),
        Password::try_from("password".to_string()).unwrap(),
        UserRoles::try_from("ADMIN,USER").expect("Invalid roles"),
        "John".to_string(),
        "last".to_string(),
        "122323223223".to_string().into(),
    );

    println!("user created: {}", String::from(user.get_roles()));

    match user.save(&mut tx).await {
        Ok(_) => println!("user saved in tx"),
        Err(e) => return Err(BoxDynError::from(format!("{} {}", String::from("asdf"), e))),
    }

    match tx.commit().await {
        Ok(_) => println!("tx commited"),
        Err(e) => return Err(BoxDynError::from(format!("{} {}", String::from("asdf"), e))),
    };

    Ok(Some(user))
}
