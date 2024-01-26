use async_trait::async_trait;
use readonly;
use sqlx::{query, Transaction};
use std::cmp::{PartialEq, PartialOrd};
use uuid::Uuid;

#[derive(PartialEq, Clone, PartialOrd, Ord, Eq, Debug)]
#[readonly::make]
pub struct UserId(String);

impl UserId {
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for UserId {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() == 32 {
            Ok(Self(s))
        } else {
            Err(()) // not a guid without hyphens
        }
    }
}

impl From<UserId> for String {
    fn from(s: UserId) -> Self {
        s.0
    }
}

#[derive(Clone, Debug)]
#[readonly::make]
pub struct UserName(String);

impl TryFrom<String> for UserName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<UserName> for String {
    fn from(n: UserName) -> Self {
        n.0
    }
}

#[cfg(test)]
impl UserName {
    pub fn pikachu() -> Self {
        Self(String::from("Pikachu"))
    }

    pub fn charmander() -> Self {
        Self(String::from("Charmander"))
    }

    pub fn bad() -> Self {
        Self(String::from(""))
    }
}

#[derive(Clone, Debug)]
#[readonly::make]
pub struct UserRoles(Vec<UserRole>);

impl TryFrom<Vec<String>> for UserRoles {
    type Error = ();

    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.iter() {
                match UserRole::try_from(String::from(t)) {
                    Ok(pt) => pts.push(pt),
                    _ => return Err(()),
                }
            }
            Ok(Self(pts))
        }
    }
}

impl From<UserRoles> for Vec<String> {
    fn from(pts: UserRoles) -> Self {
        let mut ts = vec![];
        for pt in pts.0.into_iter() {
            ts.push(String::from(pt));
        }
        ts
    }
}

#[derive(Clone, Debug)]
pub enum UserRole {
    USER,
    ADMIN,
}

impl TryFrom<String> for UserRole {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "USER" => Ok(Self::USER),
            "ADMIN" => Ok(Self::ADMIN),
            _ => Err(()),
        }
    }
}

impl From<UserRole> for String {
    fn from(t: UserRole) -> Self {
        String::from(match t {
            UserRole::USER => "USER",
            UserRole::ADMIN => "ADMIN",
        })
    }
}

#[derive(Clone, Debug)]
#[readonly::make]
pub struct Password(String);

impl Password {
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for Password {
    type Error = ();

    fn try_from(p: String) -> Result<Self, Self::Error> {
        if p.is_empty() {
            Err(())
        } else if p.len() < 8 {
            Err(())
        } else {
            Ok(Self(p))
        }
    }
}

#[derive(Debug)]
#[readonly::make]
pub struct User {
    pub id: UserId,
    pub username: UserName,
    password: Password,
    pub roles: UserRoles,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub mobile_phone: Option<String>,
    //pub address: Address,
}

impl User {
    pub fn new(username: UserName, password: Password, roles: UserRoles) -> Self {
        Self {
            id: UserId::try_from(Uuid::new_v4().simple().to_string())
                .expect("canny create user id"),
            username,
            password,
            roles,
            first_name: None,
            last_name: None,
            mobile_phone: None,
        }
    }
}

/**
fn find_by_username(username: UserName) -> Option<User>;
fn find_by_role(role: UserRole) -> Option<Vec<User>>;
fn find_all() -> Option<Vec<User>>;
*/

pub fn find_by_id(id: UserId) -> Option<User> {
    let user = User::new(
        UserName::try_from(String::from("Pikachu")).expect("bad username"),
        Password::try_from(String::from("asdfasdfasfdf")).expect("bad password"),
        UserRoles::try_from(vec![String::from("USER")]).unwrap(),
    );
    Some(user)
}

#[async_trait]
pub trait UserTrait<'a> {
    async fn save(
        self,
        tx: &'a mut Transaction<'static, sqlx::Postgres>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn delete(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
impl<'a> UserTrait<'a> for User {
    async fn save(
        self,
        tx: &'a mut Transaction<'static, sqlx::Postgres>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Saving (and consuming) user: {:?}", self.username);

        let res = query!(
            "INSERT INTO users (id, username, password, user_roles) VALUES ($1, $2, $3, $4)",
            String::from(self.id),
            String::from(self.username),
            self.password.value(),
            Vec::from(self.roles).join(",")
        )
        .execute(&mut **tx)
        .await;

        if res.is_err() {
            if res
                .unwrap_err()
                .as_database_error()
                .unwrap()
                .is_unique_violation()
            {
                println!("username already exists");
                return Err("Username already exists".into());
            }

            // TODO: handle other errors
            return Err("Insert error".into());
        }

        Ok(())
    }

    async fn delete(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::UserId;

    #[test]
    fn test_userid_short() {
        assert_eq!(
            UserId::try_from(String::from("afb46bdb7d414f5bb9ef3f2e760a4e0")),
            Err(())
        );
    }

    #[test]
    fn test_userid_long() {
        assert_eq!(
            UserId::try_from(String::from("1234567890123456789012345678901xx")),
            Err(())
        );
    }

    #[test]
    fn test_userid_ok() {
        assert_eq!(
            UserId::try_from(String::from("afb46bdb7d414f5bb9ef3f2e760a4e0e")),
            Ok(UserId(String::from("afb46bdb7d414f5bb9ef3f2e760a4e0e")))
        );
    }
}
