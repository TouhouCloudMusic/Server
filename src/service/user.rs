use std::sync::Arc;

use anyhow::{Error, Result};
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use entity::user;
use once_cell::sync::Lazy;
use sea_orm::QueryFilter;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbErr};
use sea_orm::EntityTrait;

pub enum Password {
    #[allow(dead_code)]
    Hashed(String),
    Unhashed(String),
}

pub static ARGON2_HASHER: Lazy<Argon2> = Lazy::new(Argon2::default);

impl Password {
    fn to_string(&self) -> Result<String> {
        match self {
            Password::Hashed(password) => Ok(password.to_string()),
            Password::Unhashed(password) => {
                let salt = SaltString::generate(&mut OsRng);

                let password_hash = ARGON2_HASHER
                    .hash_password(password.as_bytes(), &salt)
                    .map_err(|err| {
                        Error::msg(format!("Failed to hash password: {}", err))
                    })?;

                Ok(password_hash.to_string())
            }
        }
    }
}

impl From<&str> for Password {
    fn from(value: &str) -> Password {
        Password::Unhashed(value.to_string())
    }
}

impl From<String> for Password {
    fn from(value: String) -> Password {
        Password::Unhashed(value)
    }
}

impl From<&String> for Password {
    fn from(value: &String) -> Password {
        Password::Unhashed(value.to_string())
    }
}

#[derive(Default, Clone)]
pub struct UserService {
    database: Arc<DatabaseConnection>,
}

impl UserService {
    pub fn new(database: &Arc<DatabaseConnection>) -> Self {
        Self {
            database: Arc::clone(database),
        }
    }

    pub async fn is_exist(&self, username: &String) -> Result<bool, DbErr> {
        user::Entity::find()
            .filter(user::Column::Name.eq(username))
            .one(self.database.as_ref())
            .await
            .map(|opt| opt.is_some())
    }

    pub async fn create(
        &self,
        username: &String,
        password: Password,
    ) -> Result<user::Model> {
        let new_user = user::ActiveModel {
            name: ActiveValue::Set(username.to_string()),
            password: ActiveValue::Set(password.to_string()?),
            ..Default::default()
        };

        let user = user::Entity::insert(new_user)
            .exec_with_returning(self.database.as_ref())
            .await?;

        Ok(user)
    }

    pub async fn verify_password(
        &self,
        username: &String,
        password: &String,
    ) -> Result<user::Model> {
        if let Some(user) = user::Entity::find()
            .filter(user::Column::Name.eq(username))
            .one(self.database.as_ref())
            .await?
        {
            let parsed_hash =
                PasswordHash::new(&user.password).map_err(|op| {
                    Error::msg(format!("Failed to parse password: {}", op))
                })?;

            let verifycation_result = ARGON2_HASHER
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok();

            if verifycation_result {
                return Ok(user);
            } else {
                return Err(Error::msg("Incorrect username or password"));
            }
        }

        Err(Error::msg("User not found"))
    }

    pub async fn find_by_name(
        &self,
        username: &String,
    ) -> Result<user::Model, DbErr> {
        user::Entity::find()
            .filter(user::Column::Name.eq(username))
            .one(self.database.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound(
                "User not found after login".to_string(),
            ))
    }
}
