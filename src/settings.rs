use config::{Config, ConfigError, Environment};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{env, fmt};

lazy_static! {
  pub static ref SETTINGS: Settings = Settings::new().expect("Failed to setup settings");
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
  pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
  pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
  pub uri: String,
  pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Auth {
  pub secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
  pub environment: String,
  pub server: Server,
  pub logger: Logger,
  pub database: Database,
  pub auth: Auth,
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let mut builder = Config::builder().add_source(
      Environment::with_prefix("APP")
        .prefix_separator("_")
        .separator("__")
        .list_separator(","),
    );

    // Some cloud services like Heroku exposes a randomly assigned port in
    // the PORT env var and there is no way to change the env var name.
    if let Ok(port) = env::var("PORT") {
      builder = builder.set_override("server.port", port)?;
    }

    builder
      .build()?
      // Deserialize (and thus freeze) the entire configuration.
      .try_deserialize()
  }
}

impl fmt::Display for Settings {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "environment: {}, server: {}, logger: {}, database: {}, auth: {} ",
      &self.environment, self.server, self.logger, self.database, self.auth,
    )
  }
}

impl fmt::Display for Server {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "http://localhost:{}", &self.port)
  }
}

impl fmt::Display for Logger {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "level: {}", &self.level)
  }
}

impl fmt::Display for Database {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "uri: {}, name: {}", &self.uri, &self.name)
  }
}

impl fmt::Display for Auth {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "secret: {}", &self.secret)
  }
}
