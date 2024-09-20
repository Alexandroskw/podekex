mod data_collect;
mod db;
mod users;

use data_collect::pokemon_data::load_pokemon_data;
use db::connection::AppConfig;
use db::pokemon_tables::{insert_pokemon_data, reset_types_table};
use dotenv::dotenv;
use pokedb::users::user_config::setup_env_file;
use std::path::Path;

const TOTAL_POKEMON: u32 = 251;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
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

    // Importing the creation of the tables
    let mut config = AppConfig::new()?;
    // Init the creation of the tables
    config.init_database()?;

    reset_types_table(&mut config.db_client)?;

    for i in 1..=TOTAL_POKEMON {
        match config.fetch_pokemon(i) {
            Ok(Some(pokemon_data)) => {
                insert_pokemon_data(&mut config.db_client, &pokemon_data)?;
                println!("Inserted pokemon {i}");
            }
            Ok(None) => println!("Unavaiable to obtain data from pokemon {i}. Skkiping"),
            Err(e) => eprintln!("Error to fetching pokemon {i}: {e}"),
        }
    }

    // Making the DataFrame with Polars
    load_pokemon_data(&mut config.db_client)?;

    // Printing the DataFrame
    println!("DataFrame created.");

    Ok(())
}
