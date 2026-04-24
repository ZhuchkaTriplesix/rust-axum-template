use std::env;
use std::path::Path;
use std::time::Duration;

use anyhow::Context;
use configparser::ini::Ini;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub http: Http,
    pub postgres: Postgres,
    pub redis: Redis,
    pub docs: Docs,
    pub app: App,
}

#[derive(Debug, Clone)]
pub struct Http {
    pub host: String,
    pub port: u16,
    pub request_timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct Postgres {
    pub dsn: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone)]
pub struct Redis {
    pub url: String,
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Docs {
    pub basic_user: Option<String>,
    pub basic_password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub env: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing {0} in [{1}] in config file")]
    MissingField(&'static str, &'static str),
    #[error("read config: {0}")]
    Read(#[from] std::io::Error),
    #[error("parse: {0}")]
    Message(String),
}

impl AppConfig {
    /// Loads `config.ini` from `CONFIG_INI` / `CONFIG_PATH` or the given default path.
    pub fn from_default_path() -> anyhow::Result<Self> {
        let p = default_config_path();
        Self::load_path(p)
    }

    pub fn load_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let mut ini = Ini::new();
        ini.load(path)
            .map_err(|e| anyhow::anyhow!("{}: {}", path.display(), e))?;

        let get = |section: &str, key: &str| ini.get(section, key).map(|s| s.trim().to_string());

        let http_host = get("HTTP", "HOST").ok_or(ConfigError::MissingField("HOST", "HTTP"))?;
        let http_port: u16 = get("HTTP", "PORT")
            .as_deref()
            .unwrap_or("8000")
            .parse()
            .map_err(|e| ConfigError::Message(format!("HTTP.PORT: {e}")))?;
        let timeout_secs: u64 = get("HTTP", "REQUEST_TIMEOUT_SECS")
            .as_deref()
            .unwrap_or("30")
            .parse()
            .map_err(|e| ConfigError::Message(format!("HTTP.REQUEST_TIMEOUT_SECS: {e}")))?;

        let raw_conn = get("POSTGRES", "CONNECTION_STRING").unwrap_or_default();
        let dsn = if !raw_conn.is_empty() {
            raw_conn
        } else {
            let user =
                get("POSTGRES", "USER").ok_or(ConfigError::MissingField("USER", "POSTGRES"))?;
            let password = get("POSTGRES", "PASSWORD")
                .ok_or(ConfigError::MissingField("PASSWORD", "POSTGRES"))?;
            let ip = get("POSTGRES", "IP").ok_or(ConfigError::MissingField("IP", "POSTGRES"))?;
            let port: u16 = get("POSTGRES", "PORT")
                .as_deref()
                .unwrap_or("5432")
                .parse()
                .map_err(|e| ConfigError::Message(format!("POSTGRES.PORT: {e}")))?;
            let name =
                get("POSTGRES", "NAME").ok_or(ConfigError::MissingField("NAME", "POSTGRES"))?;
            format!("postgresql://{user}:{password}@{ip}:{port}/{name}")
        };

        let max_connections: u32 = get("POSTGRES", "MAX_CONNECTIONS")
            .as_deref()
            .unwrap_or("5")
            .parse()
            .map_err(|e| ConfigError::Message(format!("POSTGRES.MAX_CONNECTIONS: {e}")))?;

        let mut redis_url = get("REDIS", "URL").ok_or(ConfigError::MissingField("URL", "REDIS"))?;
        let password = get("REDIS", "PASSWORD").filter(|s| !s.is_empty());
        if let Some(ref pass) = password {
            if !redis_url.contains('@') {
                if redis_url.starts_with("redis://") {
                    let rest = &redis_url[8..];
                    redis_url = format!("redis://:{pass}@{rest}");
                } else {
                    redis_url = format!(
                        "redis://:{pass}@{}",
                        redis_url.trim_start_matches("redis://")
                    );
                }
            }
        }

        let doc_user = get("DOCS", "BASIC_USER").filter(|s| !s.is_empty());
        let doc_pass = get("DOCS", "BASIC_PASS").filter(|s| !s.is_empty());
        if doc_user.is_some() != doc_pass.is_some() {
            anyhow::bail!("DOCS: set both BASIC_USER and BASIC_PASS, or leave both empty");
        }

        let app_name = get("APP", "NAME").unwrap_or_else(|| "app".to_string());
        let app_env = get("APP", "ENV").unwrap_or_else(|| "development".to_string());

        Ok(Self {
            http: Http {
                host: http_host,
                port: http_port,
                request_timeout: Duration::from_secs(timeout_secs),
            },
            postgres: Postgres {
                dsn,
                max_connections,
            },
            redis: Redis {
                url: redis_url,
                password,
            },
            docs: Docs {
                basic_user: doc_user,
                basic_password: doc_pass,
            },
            app: App {
                name: app_name,
                env: app_env,
            },
        })
    }
}

fn default_config_path() -> String {
    env::var("CONFIG_INI")
        .or_else(|_| env::var("CONFIG_PATH"))
        .unwrap_or_else(|_| "config.ini".to_string())
}

/// Used by the binary: resolve path, load, and map anyhow errors to messages.
pub fn load_config() -> anyhow::Result<AppConfig> {
    let path = default_config_path();
    AppConfig::load_path(&path).with_context(|| format!("loading {path}"))
}
