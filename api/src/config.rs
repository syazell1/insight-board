use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: AppSettings,
    pub database: DatabaseSettings,
    pub jwt: JwtSettings,
}

#[derive(Deserialize, Clone)]
pub struct AppSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub client_url: String,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(Deserialize, Clone)]
pub struct JwtSettings {
    pub issuer: String,
    pub audience: String,
    pub access_token_secret: SecretString,
    pub refresh_token_secret: SecretString,
}

enum Environment {
    Local,
    Production,
}

impl DatabaseSettings {
    pub fn get_connection_options_without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn get_connection_options_with_db(&self) -> PgConnectOptions {
        self.get_connection_options_without_db()
            .database(&self.database_name)
    }
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            other => Err(format!("Invalid environment: {}", other)),
        }
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_dir = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_dir.join("configurations");

    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or("local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let env_file = format!("{}.yaml", env.as_str());

    let config = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base.yaml")))
        .add_source(config::File::from(config_dir.join(env_file)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    config.try_deserialize()
}
