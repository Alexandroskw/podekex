use polars::prelude::*;
use postgres::Client;

pub fn load_pokemon_data(client: &mut Client) -> Result<DataFrame, Box<dyn std::error::Error>> {
    /*let query = r#"
        SELECT p.*,
            string_agg(DISTINCT t.name, ', 'ORDER BY t.name) as types
        FROM pokemon p
        LEFT JOIN pokemon_types pt ON p.id = pt.pokemon_id
        LEFT JOIN types t ON pt.type_id = t.id
        GROUP BY p.id
    "#;*/

    let rows = client.query(
        "
                SELECT p.*,
                    string_agg(DISTINCT t.name, ', 'ORDER BY t.name) as types
                FROM pokemon p
                LEFT JOIN pokemon_types pt ON p.id = pt.pokemon_id
                LEFT JOIN types t ON pt.type_id = t.id
                GROUP BY p.id
            ",
        &[],
    )?;

    let mut id = Vec::new();
    let mut random_id = Vec::new();
    let mut pokedex_number = Vec::new();
    let mut name = Vec::new();
    let mut height = Vec::new();
    let mut weight = Vec::new();
    let mut hp = Vec::new();
    let mut attack = Vec::new();
    let mut defense = Vec::new();
    let mut special_attack = Vec::new();
    let mut special_defense = Vec::new();
    let mut speed = Vec::new();
    let mut types = Vec::new();

    for row in rows {
        id.push(row.get::<_, i32>("id"));
        random_id.push(row.get::<_, i32>("random_id"));
        pokedex_number.push(row.get::<_, i32>("pokedex_number"));
        name.push(row.get::<_, String>("name"));
        height.push(row.get::<_, String>("height"));
        weight.push(row.get::<_, String>("weight"));
        hp.push(row.get::<_, i32>("hp"));
        attack.push(row.get::<_, i32>("attack"));
        defense.push(row.get::<_, i32>("defense"));
        special_attack.push(row.get::<_, i32>("special_attack"));
        special_defense.push(row.get::<_, i32>("special_defense"));
        speed.push(row.get::<_, i32>("speed"));
        types.push(row.get::<_, Option<String>>("types").unwrap_or_default());
    }

    let df = DataFrame::new(vec![
        Series::new("id".into(), id),
        Series::new("random_id".into(), random_id),
        Series::new("pokedex_number".into(), pokedex_number),
        Series::new("name".into(), name),
        Series::new("height".into(), height),
        Series::new("weight".into(), weight),
        Series::new("hp".into(), hp),
        Series::new("attack".into(), attack),
        Series::new("defense".into(), defense),
        Series::new("special_attack".into(), special_attack),
        Series::new("special_defense".into(), special_defense),
        Series::new("speed".into(), speed),
        Series::new("types".into(), types),
    ])?;

    Ok(df)
}
