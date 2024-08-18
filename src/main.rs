mod db;

use db::pokemon_tables::create_tables;
use pokedb::db::connection::enable_connection;

fn main() -> Result<(), postgres::Error> {
    let mut client = enable_connection()?;

    create_tables(&mut client)?;

    Ok(())
}
