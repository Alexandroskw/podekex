mod db;
mod users;

use pokedb::db::connection::AppConfig;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(".env").exists() {
        println!(".env not found. Insert your data");
    } else {
        println!(".env found. Wait");
    }

    let mut config = AppConfig::new()?;
    config.init_database()?;

    println!("Pokedex started.");

    Ok(())
}
