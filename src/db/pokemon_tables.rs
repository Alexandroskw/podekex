use postgres::{Client, Error};
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

// Query for inserting the fetching pokemons
pub fn insert_pokemon_data(
    client: &mut Client,
    pokemon_data: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    // Obtain the first data from the API as JSON
    let pokedex_number = pokemon_data["id"].as_i64().ok_or("Missed")? as i32;
    let name = pokemon_data["name"].as_str().ok_or("Missed")?.to_string();

    /*Obtaining the height and weight as JSON. Then, the data is parsed as 64-bit float and is
    divided between 10.0. Division is for obtain the data as meters and kg*/
    let height = format!(
        "{:.2}",
        pokemon_data["height"].as_f64().ok_or("Missed")? / 10.0
    );
    let weight = format!(
        "{:.2}",
        pokemon_data["weight"].as_f64().ok_or("Missed")? / 10.0
    );

    let stats = pokemon_data["stats"].as_array().ok_or("Missed")?;
    let hp = stats[0]["base_stat"].as_i64().ok_or("Missed")? as i32;
    let attack = stats[1]["base_stat"].as_i64().ok_or("Missed")? as i32;
    let defense = stats[2]["base_stat"].as_i64().ok_or("Missed")? as i32;
    let special_attack = stats[3]["base_stat"].as_i64().ok_or("Missed")? as i32;
    let special_defense = stats[4]["base_stat"].as_i64().ok_or("Missed")? as i32;
    let speed = stats[5]["base_stat"].as_i64().ok_or("Missed")? as i32;

    client.execute(
        "INSERT INTO pokemon (pokedex_number, name, height, weight, hp, attack, defense, special_attack, special_defense, speed)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT(pokedex_number) DO UPDATE SET
            name = EXCLUDED.name,
            height = EXCLUDED.height,
            weight = EXCLUDED.weight,
            hp = EXCLUDED.hp,
            attack = EXCLUDED.attack,
            defense = EXCLUDED.defense,
            special_attack = EXCLUDED.special_attack,
            special_defense = EXCLUDED.special_defense,
            speed = EXCLUDED.speed
        RETURNING id",
        &[
            &pokedex_number,
            &name,
            &height,
            &weight,
            &hp,
            &attack,
            &defense,
            &special_attack,
            &special_defense,
            &speed
        ],
    )?;

    Ok(())
}
