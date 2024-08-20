use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{self, Path};

struct EnvConfig {
    database_url: String,
}

// Implementig the configuration for the .env file
impl EnvConfig {
    // Creating a new .env config
    fn new_env() -> Self {
        EnvConfig {
            database_url: String::new(),
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

    // Requiring the db credentials
    fn prompt(&mut self) -> io::Result<()> {
        println!("Database credentials configuration");

        let username = Self::prompt_input("Username: ")?;
        let password = Self::prompt_input("Password: ")?;
        let host = Self::prompt_input("Host (Enter for 'localhost'): ")?;
        let host = if host.is_empty() {
            "localhost".to_string()
        } else {
            host
        };
        let database = Self::prompt_input("Database name: ")?;

        self.database_url = format!(
            "postgresql://{}:{}@{}/{}",
            username, password, host, database
        );
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

        Ok(())
    }
}

pub fn setup_env_file() -> io::Result<()> {
    let mut config = EnvConfig::new_env();

    config.prompt()?;
    config.save(Path::new(".env"))?;
    println!(".env file created successful");

    Ok(())
}
