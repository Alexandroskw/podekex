use postgres::{Client, Error};

// Queries are created here
// Borrowing the client from the established connection
pub fn create_pokemon_tables(client: &mut Client) -> Result<(), Error> {
    // Query to create tables into the db
    client.batch_execute(
        "
            -- Pokemon principle table
            CREATE TABLE IF NOT EXISTS pokemon (
                id   SERIAL PRIMARY KEY,
                pokedex_number INTEGER UNIQUE NOT NULL,
                name    VARCHAR(100) NOT NULL,
                height  VARCHAR(10),
                weight  VARCHAR(10),
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
            -- Abilities of pokemon table
            CREATE TABLE IF NOT EXISTS abilities (
                id SERIAL PRIMARY KEY,
                name VARCHAR(50) UNIQUE NOT NULL
        );
            -- Abilities from each pokemon table
            CREATE TABLE IF NOT EXISTS pokemon_abilities (
                pokemon_id INTEGER REFERENCES pokemon(id),
                ability_id INTEGER REFERENCES abilities(id),
                is_hidden BOOLEAN NOT NULL,
                PRIMARY KEY (pokemon_id, ability_id)
        );

        ",
    )?;

    println!("Tables created or uploaded.");

    Ok(())
}
