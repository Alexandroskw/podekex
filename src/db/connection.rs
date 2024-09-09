use crate::{db::pokemon_tables::create_pokemon_tables, users::user_config::setup_env_file};
use postgres::{Client, Error, NoTls};
use reqwest::Client as ReqwestClient;
use serde_json::Value;
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
        let db_client = Self::enable_connection()?;
        let (api_client, api_base_url) = Self::config_pokemon_api()?;

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

    // Connecting to the database URL
    fn enable_connection() -> Result<Client, Error> {
        // Get the databse URL from the environment variable
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file. Denied");
        // Establishing connection
        Client::connect(&database_url, NoTls)
    }

    fn config_pokemon_api() -> Result<(ReqwestClient, String), Box<dyn std::error::Error>> {
        let api_base_url = env::var("POKEMON_BASE_API_URL")
            .expect("POKEMON_BASE_API_URL must be set in .env file");

        let client = ReqwestClient::new();

        Ok((client, api_base_url))
    }

    pub fn fetch_pokemon(
        &self,
        pokemon_id: u32,
    ) -> Result<Option<Value>, Box<dyn std::error::Error>> {
        // Connecting with the .env variable
        let api_base_url = env::var("POKEMON_BASE_API_URL")
            .expect("POKEMON_BASE_API_URL must be set in .env file.");
        // Like the postgres crate uses 'Client' too, at compiling show an error. Just avoiding that
        let client = reqwest::blocking::Client::new();
        let url = format!("{}{}", api_base_url, pokemon_id);
        // Obtain a GET method for the HTTP request
        let response = client.get(&url).send()?;

        // Enabling connection with the API data
        if response.status().is_success() {
            let pokemon_data = response.json::<Value>()?;
            Ok(Some(pokemon_data))
        } else {
            eprintln!("Error fetching pokemon {}{}", pokemon_id, response.status());
            Ok(None)
        }
    }
}
