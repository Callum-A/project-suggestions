use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub database_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub google_scopes: Vec<String>,
    pub google_token_url: String,
    pub google_user_info_url: String,
    pub jwt_secret: String,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnvVar(String),
}

impl Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingEnvVar(var) => write!(f, "missing environment variable: {}", var),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Config, ConfigError> {
        let host = std::env::var("HOST").map_err(|_| ConfigError::MissingEnvVar("HOST".into()))?;
        let port = std::env::var("PORT").map_err(|_| ConfigError::MissingEnvVar("PORT".into()))?;
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".into()))?;
        let google_client_id = std::env::var("GOOGLE_CLIENT_ID")
            .map_err(|_| ConfigError::MissingEnvVar("GOOGLE_CLIENT_ID".into()))?;
        let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
            .map_err(|_| ConfigError::MissingEnvVar("GOOGLE_CLIENT_SECRET".into()))?;
        let google_redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
            .map_err(|_| ConfigError::MissingEnvVar("GOOGLE_REDIRECT_URI".into()))?;
        let google_scopes = std::env::var("GOOGLE_SCOPES")
            .map_err(|_| ConfigError::MissingEnvVar("GOOGLE_SCOPES".into()))?
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let google_token_url = std::env::var("GOOGLE_TOKEN_URL")
            .map_err(|_| ConfigError::MissingEnvVar("GOOGLE_TOKEN_URL".into()))?;
        let google_user_info_url = std::env::var("GOOGLE_USER_INFO_URL")
            .map_err(|_| ConfigError::MissingEnvVar("GOOGLE_USER_INFO_URL".into()))?;
        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| ConfigError::MissingEnvVar("GOOGLE_USER_INFO_URL".into()))?;

        Ok(Config {
            host,
            port,
            database_url,
            google_client_id,
            google_client_secret,
            google_redirect_uri,
            google_scopes,
            google_token_url,
            google_user_info_url,
            jwt_secret,
        })
    }
}
