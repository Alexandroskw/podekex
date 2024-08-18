use postgres::{Client, Error, NoTls};

fn enable_connection() -> Result<Client, Error> {
    let conn_string = "host=localhost user=alex_desk password=alex dbname=pokedex";
    Client::connect(conn_string, NoTls)
}

fn main() -> Result<(), postgres::Error> {
    let mut client = enable_connection()?;
    client.batch_execute(
        "
            CREATE TABLE IF NOT EXISTS pokemon (
                id      INTEGER PRIMARY KEY,
                pokedex_number INTEGER UNIQUE NOT NULL,
                name    VARCHAR(100) NOT NULL,
                height  DECIMAL(5,2)
        )",
    )?;

    println!("Table created.");

    Ok(())
}
