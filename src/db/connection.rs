use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use std::env;

// Connecting to the database
pub fn enable_connection() -> Result<Client, Error> {
    // Load environment variables from the .env
    dotenv().ok();

    // Get the URL from the environment variable
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file. Denied");

    // Establishing connection
    Client::connect(&database_url, NoTls)
}
