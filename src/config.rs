use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use deadpool_postgres::{Config, ManagerConfig, Pool};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigData {
    pub db_login: String,
    pub db_password: String,
    pub db_name: String,
    pub db_host: String,
    pub db_port: u16,
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

#[derive(Clone)]
pub struct ServerConfig {
    pub cd: ConfigData,
    pub pg: Pool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let cd = ConfigData {
            db_login: "postgres".to_string(),
            db_password: "1234".to_string(),
            db_name: "default".to_string(),
            db_host: "localhost".to_string(),
            db_port: 5432,
            host: "localhost".to_string(),
            port: 8080,
            log_level: "Debug".to_string(),
        };

        let mut pg_conf = Config::new();
        pg_conf.dbname = Some(cd.db_name.clone());
        pg_conf.host = Some(cd.db_host.clone());
        pg_conf.user = Some(cd.db_login.clone());
        pg_conf.password = Some(cd.db_login.clone());
        pg_conf.port = Some(cd.db_port);
        pg_conf.manager = Some(ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        });
        let pool = pg_conf
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .unwrap();

        Self { cd, pg: pool }
    }
}

impl ServerConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut file: File;
        let path = Path::new("config.toml");
        let mut raw = String::new();
        let mut data: ServerConfig;

        if !path.exists() {
            file = File::create(path)?;
            data = ServerConfig::default();

            raw = toml::to_string_pretty(&data.cd)?;
            file.write_all(raw.as_bytes())?;

            return Ok(data);
        }

        file = File::open(path)?;

        file.read_to_string(&mut raw)?;

        data = ServerConfig::default();
        data.cd = toml::from_str::<ConfigData>(&raw)?;

        let mut pg_conf = Config::new();
        pg_conf.dbname = Some(data.cd.db_name.clone());
        pg_conf.host = Some(data.cd.db_host.clone());
        pg_conf.user = Some(data.cd.db_login.clone());
        pg_conf.password = Some(data.cd.db_password.clone());
        pg_conf.port = Some(data.cd.db_port);
        pg_conf.manager = Some(ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        });
        let pool = pg_conf
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .unwrap();

        data.pg = pool;

        Ok(data)
    }
}
