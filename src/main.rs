mod db;
mod users;

use pokedb::users::user_config::setup_env_file;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*If the '.env' file doesn't exists, the Pokedex will send the user to configure it's
    credentials. But if the .'env' exists, the pokedex will send a message that everything is
    correct and the pokedex has been init*/
    if !Path::new(".env").exists() {
        println!(".env not found. Insert your data");
        // Here the user will set the correct data for the db
        setup_env_file()?;
    } else {
        println!(".env found. Wait");
    }

    println!("Pokedex started.");

    Ok(())
}
