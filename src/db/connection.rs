use postgres::{Client, Error, NoTls};

// Connecting to the database
pub fn enable_connection() -> Result<Client, Error> {
    let conn_string = "host=localhost user=alex_desk password=alex dbname=pokedex";
    Client::connect(conn_string, NoTls)
}
