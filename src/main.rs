mod db;
mod users;

use db::pokemon_tables::init_pokemon_database;
use pokedb::db::connection::AppConfig;
use users::user_config::setup_env_file;

fn main() -> Result<(), postgres::Error> {
    let mut configs = AppConfig::new()?;

    match setup_env_file() {
        Ok(_) => println!("Configuration complete."),
        Err(e) => eprintln!("Error to create .env file: {}", e),
    }

    init_pokemon_database(&mut client)?;

    Ok(())
}
