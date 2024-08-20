use crate::db::connection::enable_connection;
use postgres::{Client, Error};

// Queries are created here
// Borrowing the client from the established connection
fn create_pokemon_tables(client: &mut Client) -> Result<(), Error> {
    // Query to create tables into the db
    client.batch_execute(
        "
            -- Droping tables if exists
            DROP TABLE IF EXISTS pokemon;
            DROP TABLE IF EXISTS types;
            DROP TABLE IF EXISTS pokemon_types;
            DROP TABLE IF EXISTS abilities;
            DROP TABLE IF EXISTS pokemon_abilities;

            -- Pokemon principle table
            CREATE TABLE IF NOT EXISTS pokemon (
                id   SERIAL PRIMARY KEY,
                pokedex_number INTEGER UNIQUE NOT NULL,
                name    VARCHAR(100) NOT NULL,
                height  DECIMAL(5,2),
                weight  DECIMAL(5,2),
                hp INTEGER,
                attack INTEGER,
                defense INTEGER,
                special_attack INTEGER,
                special_defense INTEGER,
                speed INTEGER
        );
            -- Types of pokemon table
            CREATE TABLE IF NOT EXISTS types (
                id SERIAL PRIMARY KEY,
                name VARCHAR(20)
        );
            -- Pokemon types table
            CREATE TABLE IF NOT EXISTS pokemon_types (
                pokemon_id SERIAL REFERENCES pokemon(id),
                type_id INTEGER REFERENCES types(id),
                PRIMARY KEY (pokemon_id, type_id)
            );
            CREATE TABLE IF NOT EXISTS abilities (
                id SERIAL PRIMARY KEY,
                name VARCHAR(50) UNIQUE NOT NULL
            );
            CREATE TABLE IF NOT EXISTS pokemon_abilities (
                pokemon_id UUID REFERENCES pokemon(id),
                ability_id INTEGER REFERENCES abilities(id),
                is_hidden BOOLEAN NOT NULL,
                PRIMARY KEY (pokemon_id, ability_id)
            );

        ",
    )?;

    println!("Tables created.");

    Ok(())
}

// Initialize the pokemon database
pub fn init_pokemon_database(client: &mut Client) -> Result<(), Error> {
    let mut client = enable_connection()?;

    // Confirms the creation of the tables in the db
    create_pokemon_tables(&mut client)?;
    println!("Pokemon database initalized successful");

    Ok(())
}
