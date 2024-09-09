use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

const POKE_API_URL: &str = "https://pokeapi.co/api/v2/pokemon/";

#[allow(dead_code)]
struct EnvConfig {
    database_url: String,
    api_base_url: String,
}

// Implementig the configuration for the .env file
#[allow(dead_code)]
impl EnvConfig {
    // Creating a new .env config
    fn new_env() -> Self {
        EnvConfig {
            database_url: String::new(),
            api_base_url: String::new(),
        }
    }

    // Input in the prompt
    fn prompt_input(message: &str) -> io::Result<String> {
        print!("{}", message);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }

    // Requiring the db credentials from the user
    fn prompt(&mut self) -> io::Result<()> {
        println!("Database credentials configuration");

        // Requesting the username, password and the host from the user to enable connection with
        // the .env file
        let username = Self::prompt_input("Username: ")?;
        let password = Self::prompt_input("Password: ")?;
        let host = Self::prompt_input("Host (Enter for 'localhost'): ")?;

        // If the user don't set any host, the host will be 'localhost' by default

        // if the user press the enter key, by default, 'localhost' is setting in the .env file.
        // Otherwise, the host will be set the one the user gives

        let host = if host.is_empty() {
            "localhost".to_string()
        } else {
            host
        };

        let database = Self::prompt_input("Database name: ")?;
        let poke_api_url = Self::prompt_input(
            "Pokemon API URL (Enter for 'https://pokeapi.co/api/v2/pokemon/'): ",
        )?;

        // If the user don't set any URL of Pokemon API, the Pokedex will set automatically
        if poke_api_url.is_empty() {
            POKE_API_URL.to_string()
        } else {
            poke_api_url
        };


        // Printing the DB URL in the .env

        // The db URL will be set with the data of the user

        self.database_url = format!(
            "postgresql://{}:{}@{}/{}",
            username, password, host, database
        );

        // Printing the URL Pokemon API in the .env file
        self.api_base_url = format!("{}", POKE_API_URL);

        Ok(())
    }

    // Saving the .env file
    fn save(&self, path: &Path) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        writeln!(file, "DATABASE_URL={}", self.database_url)?;
        writeln!(file, "POKEMON_BASE_API_URL={}", self.api_base_url)?;

        Ok(())
    }
}


#[allow(dead_code)]
// This function saves the .env file with the user credentials and creates it if not exists
pub fn setup_env_file() -> io::Result<()> {
    let mut config = EnvConfig::new_env();

    config.prompt()?;
    config.save(Path::new(".env"))?;
    println!(".env file created successfully");

    Ok(())
}