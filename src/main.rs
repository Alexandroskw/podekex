mod db;
mod users;

use db::pokemon_tables::init_pokemon_database;
use pokedb::db::connection::enable_connection;
use users::user_config::setup_env_file;

fn main() -> Result<(), postgres::Error> {
    match setup_env_file() {
        Ok(_) => println!("Configuration complete."),
        Err(e) => eprintln!("Error to create .env file: {}", e),
    }

    let mut client = enable_connection()?;

    init_pokemon_database(&mut client)?;

    Ok(())
}
