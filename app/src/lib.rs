mod db;
mod entities;
use sqlx::error::BoxDynError;
use sqlx::Transaction;

use entities::user::{CreateUserDto, Password, User, UserName, UserRoles, UserTrait};

pub async fn execute() -> Result<Option<User>, Box<dyn std::error::Error>> {
    println!("Starting User test");

    let db = db::connect().await;

    sqlx::query!("TRUNCATE TABLE users")
        .execute(&db)
        .await
        .expect("truncate table users");

    let mut tx: Transaction<'static, sqlx::Postgres> = db.begin().await.unwrap();

    let user = User::new(CreateUserDto {
        username: UserName::try_from("John".to_string()).expect("Invalid username"),
        password: Password::try_from("password".to_string()).unwrap(),
        roles: UserRoles::try_from("ADMIN,USER").expect("Invalid roles"),
        first_name: "John".to_string(),
        last_name: "last".to_string(),
        mobile_phone: "122323223223".to_string().into(),
    });

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
