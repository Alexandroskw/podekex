mod db;

use db::pokemon_tables::init_pokemon_database;
use pokedb::db::connection::enable_connection;

fn main() -> Result<(), postgres::Error> {
    let mut client = enable_connection()?;

    init_pokemon_database(&mut client)?;

    Ok(())
}
