use crate::{db::pokemon_tables::create_pokemon_tables, users::user_config::setup_env_file};
use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use reqwest::Client as ReqwestClient;
use std::env;

// Struct for the Pokemon API and the client of the DB
#[allow(dead_code)]
pub struct AppConfig {
    pub db_client: Client,
    pub api_client: ReqwestClient,
    pub api_base_url: String,
}

// Encapsulated all the configurations
#[allow(dead_code)]
impl AppConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load environment variables from the .env
        dotenv().ok();

        let db_client = enable_connection()?;
        let (api_client, api_base_url) = config_pokemon_api()?;

        Ok(AppConfig {
            db_client,
            api_client,
            api_base_url,
        })
    }

    pub fn init_database(&mut self) -> Result<(), Error> {
        create_pokemon_tables(&mut self.db_client)
    }

    pub fn setup_env() -> Result<(), Box<dyn std::error::Error>> {
        setup_env_file()?;

        Ok(())
    }
}

// Connecting to the database
fn enable_connection() -> Result<Client, Error> {
    // Get the URL from the environment variable
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file. Denied");

    // Establishing connection
    Client::connect(&database_url, NoTls)
}

fn config_pokemon_api() -> Result<(ReqwestClient, String), Box<dyn std::error::Error>> {
    let api_base_url =
        env::var("POKEMON_API_BASE_URL").expect("POKEMON_API_BASE_URL must be set in .env file");

    let client = ReqwestClient::new();

    Ok((client, api_base_url))
}
