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
            -- Pokemon types table
            CREATE TABLE IF NOT EXISTS types (
                id SERIAL PRIMARY KEY,
                name VARCHAR(20)
        );
        ",
    )?;

    println!("Tables created.");

    Ok(())
}

// Initialize the pokemon database
pub fn init_pokemon_database(client: &mut Client) -> Result<(), Error> {
    // Confirms the creation of the tables in the db
    create_pokemon_tables(client)?;
    println!("Pokemon database initalized successful");

    Ok(())
}
