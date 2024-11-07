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
use regex::Regex;
use sea_orm::{
    prelude::Expr, sea_query::Query, ConnectionTrait, DatabaseBackend,
    EntityTrait,
};
use sea_orm::{sea_query::Alias, QueryFilter};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbErr};

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
                        Error::msg(format!("Failed to hash password: {err}"))
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
    database: DatabaseConnection,
}

impl UserService {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }

    pub async fn is_exist(
        &self,
        username: &String,
    ) -> Result<bool, anyhow::Error> {
        const ALIAS: &str = "is_exist";
        let query = Query::select()
            .expr_as(
                Expr::exists(
                    Query::select()
                        .expr(Expr::value(1))
                        .from(user::Entity)
                        .and_where(user::Column::Name.eq(username))
                        .to_owned(),
                ),
                Alias::new(ALIAS),
            )
            .to_owned();

        let stmt = DatabaseBackend::Postgres.build(&query);

        if let Some(result) = self.database.query_one(stmt).await? {
            let is_exist: bool = result.try_get_by(ALIAS)?;

            return Ok(is_exist);
        }

        Err(Error::msg("Failed to check if user exists"))
    }

    pub async fn create(
        &self,
        username: &String,
        password: Password,
    ) -> Result<user::Model> {
        if !validate_username(username) {
            return Err(Error::msg("Invalid username"));
        }

        let new_user = user::ActiveModel {
            name: ActiveValue::Set(username.to_string()),
            password: ActiveValue::Set(password.to_string()?),
            ..Default::default()
        };

        let user = user::Entity::insert(new_user)
            .exec_with_returning(&self.database)
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
            .one(&self.database)
            .await?
        {
            let parsed_hash =
                PasswordHash::new(&user.password).map_err(|op| {
                    Error::msg(format!("Failed to parse password: {op}"))
                })?;

            let verification_result = ARGON2_HASHER
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok();

            if verification_result {
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
    ) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Name.eq(username))
            .one(&self.database)
            .await
    }
}

fn validate_username(username: &str) -> bool {
    static USER_NAME_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[\p{L}\p{N}_]{1,32}$").unwrap());

    if !USER_NAME_REGEX.is_match(username) {
        return false;
    }

    !username
        .chars()
        .any(|c| c.is_control() || c.is_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        let test_cases = [
            // 长度
            ("", false),
            (&"a".repeat(33), false),
            // 空格
            (" a ", false),
            ("a a", false),
            // 特殊字符
            ("😀", false),       // emoji
            (" ", false),        // 单个空格
            ("\n", false),       // 换行符
            ("\t", false),       // 制表符
            ("\u{200B}", false), // 零宽空格
            ("\u{00A0}", false), // 不间断空格
            ("alice_megatron", true),
            // 中文
            ("无蛋黄", true),
            ("憂鬱的臺灣烏龜", true),
            // 日文
            ("ひらがな", true),
            ("かたかな", true),
            ("カタカナ", true),
            // 韩文
            ("안녕하세요", true),
            ("사용자", true),
            // 西里尔字母
            ("пример", true),
            ("пользователь", true),
            // 德语字符
            ("müller", true),
            ("straße", true),
            // 阿拉伯字符
            ("مرحبا", true),
            ("مستخدم", true),
        ];

        for (username, expected) in test_cases {
            assert_eq!(validate_username(username), expected);
        }
    }
}
