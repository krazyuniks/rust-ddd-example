use async_trait::async_trait;
use sqlx::{query, Transaction};

pub async fn connect() -> sqlx::PgPool {
    let conn_str = String::from("postgresql://postgres:password@localhost/users");
    let conn_pool = sqlx::PgPool::connect(&conn_str).await;

    match conn_pool {
        Ok(_) => println!("Connection established"),
        Err(e) => panic!("Connection error: {}", e),
    }

    conn_pool.unwrap()
}

/*
pub async fn db() -> Result<(), Box<dyn std::error::Error>> {
    let pool = pgsql_db_pool().await;
    let _ = query!(r"TRUNCATE TABLE users").execute(&pool).await?;

    let mut uow: Transaction<'static, sqlx::Postgres> = pool.begin().await.unwrap();

    let mut user_repo = UserRepo::new(&mut uow).await.unwrap();
    user_repo
        .insert(User {
            id: 1,
            name: format!("name:{}", 1),
        })
        .await
        .unwrap();

    match uow.commit().await {
        Ok(_) => println!("Transaction committed"),
        Err(e) => println!("Transaction error: {}", e),
    };

    let inserted_user = query!(r#"SELECT id, name FROM users WHERE id = $1"#, 1)
        .fetch_one(&pool)
        .await;
    assert!(inserted_user.is_ok());

    let users: Vec<User> = sqlx::query_as!(User, r"SELECT id, name FROM users")
        .fetch_all(&pool)
        .await?;

    users.iter().for_each(|user| {
        print!("{:#?}", user);
    });

    Ok(())
}

#[derive(sqlx::FromRow, Debug)]
struct User {
    id: i64,
    name: String,
}
struct UserRepo<'a> {
    tx: &'a mut Transaction<'static, sqlx::Postgres>,
}

#[async_trait]
trait RepoTrait<'a, T, U = Self> {
    async fn new(
        tx: &'a mut Transaction<'static, sqlx::Postgres>,
    ) -> Result<U, Box<dyn std::error::Error + Send + Sync>>;
    async fn insert(&mut self, entity: T) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn get(&mut self, id: i64) -> Result<T, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_all(&mut self) -> Result<Vec<T>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
impl<'a> RepoTrait<'a, User, UserRepo<'a>> for UserRepo<'a> {
    async fn new(
        tx: &'a mut Transaction<'static, sqlx::Postgres>,
    ) -> Result<UserRepo<'a>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(UserRepo { tx })
    }

    async fn insert(&mut self, user: User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        query!(
            r#"INSERT INTO users (id, name) VALUES ( $1, $2 )"#,
            user.id,
            user.name,
        )
        // In 0.7, `Transaction` can no longer implement `Executor` directly,
        // so it must be dereferenced to the internal connection type.
        .execute(&mut **self.tx)
        .await?;

        println!("insert user(): {:?}", user);
        Ok(())
    }

    async fn get(&mut self, id: i64) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let row = query!(r#"SELECT id, name FROM users WHERE id = $1"#, id)
            .fetch_one(&mut **self.tx)
            .await?;

        Ok(User {
            id: row.id,
            name: row.name,
        })
    }

    async fn get_all(&mut self) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let users: Vec<User> = sqlx::query_as!(User, r"SELECT id, name FROM users ORDER BY id")
            .fetch_all(&mut **self.tx)
            .await?;

        Ok(users)
    }
}
*/
