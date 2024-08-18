use postgres::{Client, Error};

// Queries are created here
// Borrowing the client from the established connection
pub fn create_tables(client: &mut Client) -> Result<(), Error> {
    // Query to create tables into the db
    client.batch_execute(
        "
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
            CREATE IF NOT EXISTS types (
                id SERIAL PRIMARY KEY,
                name VARCHAR(20)
        );
        ",
    )?;

    println!("Tables created.");

    Ok(())
}
