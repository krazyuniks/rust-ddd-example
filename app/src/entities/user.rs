use async_trait::async_trait;
use sqlx::error::BoxDynError;
use sqlx::{query, Transaction};
use std::cmp::{PartialEq, PartialOrd};
use std::format;
use uuid::Uuid;

#[non_exhaustive]
#[derive(PartialEq, Clone, PartialOrd, Ord, Eq, Debug)]
pub struct UserId(String);

impl UserId {
    fn new() -> Self {
        UserId(Uuid::new_v4().simple().to_string())
    }
}

impl From<&UserId> for String {
    fn from(s: &UserId) -> Self {
        s.0.clone()
    }
}

impl From<UserId> for String {
    fn from(s: UserId) -> Self {
        s.0
    }
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct UserName(String);

impl TryFrom<String> for UserName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() || n.len() > 255 || n.len() < 4 {
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

impl From<&UserName> for String {
    fn from(n: &UserName) -> Self {
        n.0.clone()
    }
}

#[non_exhaustive]
#[derive(Clone, Debug)]
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

impl TryFrom<&str> for UserRoles {
    type Error = ();

    fn try_from(ts: &str) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.split(",") {
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

impl From<&UserRoles> for String {
    fn from(pts: &UserRoles) -> Self {
        let mut ts = vec![];
        for pt in pts.0.iter() {
            ts.push(String::from(pt));
        }
        ts.join(",")
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

impl From<&UserRole> for String {
    fn from(t: &UserRole) -> Self {
        String::from(match t {
            UserRole::USER => "USER",
            UserRole::ADMIN => "ADMIN",
        })
    }
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Password(String);

impl From<Password> for String {
    fn from(p: Password) -> Self {
        p.0
    }
}

impl From<&Password> for String {
    fn from(p: &Password) -> Self {
        p.0.clone()
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

#[non_exhaustive]
#[derive(Debug)]
pub struct User {
    id: UserId,
    username: UserName,
    password: Password,
    roles: UserRoles,
    first_name: String,
    last_name: String,
    mobile_phone: Option<String>,
}

impl User {
    pub fn new(
        username: UserName,
        password: Password,
        roles: UserRoles,
        first_name: String,
        last_name: String,
        mobile_phone: Option<String>,
    ) -> Self {
        User {
            id: UserId::new(),
            username,
            password,
            roles,
            first_name,
            last_name,
            mobile_phone,
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.0.as_str()
    }

    pub fn get_username(&self) -> &str {
        self.username.0.as_str()
    }

    pub fn get_mobile_phone(&self) -> &str {
        match &self.mobile_phone {
            Some(p) => p,
            None => "",
        }
    }

    pub fn get_roles(&self) -> String {
        String::from(&self.roles)
    }

    pub fn get_password(&self) -> &str {
        &self.password.0.as_str()
    }
}

/**
fn find_by_username(username: UserName) -> Option<User>;
fn find_by_role(role: UserRole) -> Option<Vec<User>>;
fn find_all() -> Option<Vec<User>>;
fn find_by_id() -> Option<Vec<User>>;
*/

#[async_trait]
pub trait UserTrait<'a> {
    async fn save(
        &self,
        tx: &'a mut Transaction<'static, sqlx::Postgres>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn delete(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
impl<'a> UserTrait<'a> for User {
    async fn save(
        &self,
        tx: &'a mut Transaction<'static, sqlx::Postgres>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Saving (and consuming) user: {:?}", self.username);

        let uname = String::from(self.username.clone());

        let res = query!(
            "INSERT INTO users (
                id, 
                username, 
                password, 
                user_roles, 
                first_name, 
                last_name, 
                mobile_phone
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &self.get_id(),
            &self.get_username(),
            &self.get_password(),
            &self.get_roles(),
            &self.first_name,
            &self.last_name,
            &self.get_mobile_phone(),
        )
        .execute(&mut **tx)
        .await;

        if res.is_err() {
            if res
                .as_ref()
                .unwrap_err()
                .as_database_error()
                .unwrap()
                .is_unique_violation()
            {
                return Err(BoxDynError::from(format!(
                    "Username \"{}\" already exists, db error: {:#?}",
                    uname, res
                )));
            }

            return Err(BoxDynError::from(format!("Unknown DB error: {:#?}", res)));
        }

        Ok(())
    }

    async fn delete(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

#[cfg(test)]
mod test {}
