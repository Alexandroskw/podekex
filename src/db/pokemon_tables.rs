use postgres::{row, Client, Error, GenericClient};
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

    let types = pokemon_data["types"]
        .as_array()
        .ok_or("Missing types array")?;

    // The cycle inserts the last data into the other tables
    for type_pokemon in types {
        // Fetching the pokemon type by 'type' and the name of the type like "bug" or "fire"
        let type_name = type_pokemon["type"]["name"]
            .as_str()
            .ok_or("Missing type name")?;

        // Inserting the pokemon type on the 'types' table
        client.execute(
            "INSERT INTO types (name) VALUES ($1) ON CONFLICT DO NOTHING",
            &[&type_name],
        )?;

        /*Using 'query' instead of 'execute' for the query type. Next, we selecting the id column
        from the 'types' table and insert the type into the table*/
        let rows = client.query("SELECT id FROM types WHERE name = $1", &[&type_name])?;

        // after fetching the id of the types, It's asigned a 32-bit integer and get the row 0
        // from the JSON with the 'get' method of http
        let type_id: i32 = if let Some(row) = rows.first() {
            row.get(0)
        } else {
            return Err("Type not found".into());
        };

        /*Like the rows variable, it's fetching the id from pokemon table and inserting the pokedex
        number like the id in the table*/
        let pokemon_rows = client.query(
            "SELECT id FROM pokemon WHERE pokedex_number = $1",
            &[&pokedex_number],
        )?;
        let pokemon_id: i32 = if let Some(row) = pokemon_rows.first() {
            row.get(0)
        } else {
            return Err("Pokemon not found".into());
        };

        // Once fetched the data, the pokedex insert the id type and pokemon id in the
        // pokemon_types table
        client.execute(
            "INSERT INTO pokemon_types (pokemon_id, type_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            &[&pokemon_id, &type_id],
        )?;
    }

    Ok(())
}
