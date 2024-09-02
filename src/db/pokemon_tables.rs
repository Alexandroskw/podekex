use postgres::{Client, Error, GenericClient};
use serde_json::Value;

// Queries are created here
// Borrowing the client from the established connection
pub fn create_pokemon_tables(client: &mut Client) -> Result<(), Error> {
    // Query to create tables into the db
    client.batch_execute(
        "
            -- Pokemon principle table
            CREATE TABLE IF NOT EXISTS pokemon (
                id   SERIAL PRIMARY KEY,
                random_id   SERIAL UNIQUE NOT NULL,
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

// Query for inserting the fetching pokemons
pub fn insert_pokemon_data(client: &mut Client, pokemon_data: &Value) -> Result<(), Error> {
    // Obtain the first data from the API as JSON
    let id = pokemon_data["id"].as_i64().unwrap_or(0) as i32;
    let name = pokemon_data["name"].as_str().unwrap_or("Unknown");
    let height = pokemon_data["height"].as_f64().unwrap_or(0.0);
    let weight = pokemon_data["weight"].as_f64().unwrap_or(0.0);

    let binding = Vec::new();
    let stats = pokemon_data["stats"].as_array().unwrap_or(&binding);
    let mut hp = 0;
    let mut attack = 0;
    let mut defense = 0;
    let mut special_attack = 0;
    let mut special_defense = 0;
    let mut speed = 0;

    for stat in stats {
        let stat_name = stat["stat"]["name"].as_str().unwrap_or("");
        let base_stat = stat["base_stat"].as_i64().unwrap_or(0) as i32;
        match stat_name {
            "hp" => hp = base_stat,
            "attack" => attack = base_stat,
            "defense" => defense = base_stat,
            "special_attack" => special_attack = base_stat,
            "special_defense" => special_defense = base_stat,
            "speed" => speed = base_stat,
            _ => {}
        }
    }

    client.execute("INSERT INTO pokemon (pokedex_number, name, height, weight, hp, attack, defense, special_attack, special_defense, speed)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", &[&id, &name, &height, &weight, &hp, &attack, &defense, &special_attack, &special_defense, &speed],)?;

    Ok(())
}
