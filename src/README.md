# Pokedex
This is Pokedex uses the [Poke API](https://www.pokeapi.co), and **PostgreSQL**. This Pokedex obtain the most common
Pokemon types, the distributions of their Height, Weight, Speed, HP, Special attack, Special defense and Defense
Plotting in a PNG Images.

## How to use
-------------
For use this pokedex you need to install the Rust compiler and PostgreSQL. For Rust compiler you can install it from
their own [website](https://www.rust-lang.org/tools/install) and for PostgreSQL you can install it with the package
manager from your distro, for example Fedora
```
sudo dnf install postgresql-server postgres-contrib
```
> [!NOTE]
> For the usage of the `Plotters` crate, the crate documentation says you need to install `pk-config`, `libfreetype-6-dev`
> and `libfontconfig1-dev`. Ensure to install it.

After the installation of Postgres, create a new DB (I suggest _Pokedex_) before you running the app.

Now you can compile and run the program
```
cargo build && cargo run
```
When you run it for the first time, it will require some information like:
    - DB name
    - Password
    - URL API
By default, the URL will be https://www.pokeapi.co/api/v2/pokemon
When you finishing to set the data, it will create a hidden **env**. After that, the pokedex will show you an error
but it's ok, you can run again with `cargo run` and the Pokedex will start to fetch the Pokemon in the tables.
If you check the fetched Pokemon in the DB, you will see they are in disorder but don't worry __ALL__ of the Pokemon
are in the DB you can use a SQL query
```
SELECT *
FROM pokemon
WHERE pokedex_number <= 150
ORDER BY pokedex_number
LIMIT 150
```
And the DB will show you the first generation of Pokemon or you can search for a generation in particular
```
SELECT
   CASE
       WHEN pokedex_number BETWEEN 151 AND 251 THEN '2nd Generation'
   END AS generacion,
   *
FROM pokemon
WHERE pokedex_number BETWEEN 151 AND 251
ORDER BY pokedex_number;
```

This pokedex is for fun, so Have fun!
