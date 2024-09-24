use plotters::prelude::*;
use polars::prelude::*;
use postgres::Client;
use std::error::Error;

// Adding a struct for more control in the creation of the Vectors
struct PokemonAttribs {
    id: Vec<i32>,
    random_id: Vec<i32>,
    pokedex_number: Vec<i32>,
    name: Vec<String>,
    height: Vec<String>,
    weight: Vec<String>,
    hp: Vec<i32>,
    attack: Vec<i32>,
    defense: Vec<i32>,
    special_attack: Vec<i32>,
    special_defense: Vec<i32>,
    speed: Vec<i32>,
    types: Vec<String>,
}

pub fn load_pokemon_data(client: &mut Client) -> Result<DataFrame, Box<dyn std::error::Error>> {
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

    // Vectors for the creation of the DataFrame
    let mut pokemon_attribs = PokemonAttribs {
        id: Vec::new(),
        random_id: Vec::new(),
        pokedex_number: Vec::new(),
        name: Vec::new(),
        height: Vec::new(),
        weight: Vec::new(),
        hp: Vec::new(),
        attack: Vec::new(),
        defense: Vec::new(),
        special_attack: Vec::new(),
        special_defense: Vec::new(),
        speed: Vec::new(),
        types: Vec::new(),
    };

    // Fill the vectors with the data of the DB
    for row in rows {
        pokemon_attribs.id.push(row.get::<_, i32>("id"));
        pokemon_attribs
            .random_id
            .push(row.get::<_, i32>("random_id"));
        pokemon_attribs
            .pokedex_number
            .push(row.get::<_, i32>("pokedex_number"));
        pokemon_attribs.name.push(row.get::<_, String>("name"));
        pokemon_attribs.height.push(row.get::<_, String>("height"));
        pokemon_attribs.weight.push(row.get::<_, String>("weight"));
        pokemon_attribs.hp.push(row.get::<_, i32>("hp"));
        pokemon_attribs.attack.push(row.get::<_, i32>("attack"));
        pokemon_attribs.defense.push(row.get::<_, i32>("defense"));
        pokemon_attribs
            .special_attack
            .push(row.get::<_, i32>("special_attack"));
        pokemon_attribs
            .special_defense
            .push(row.get::<_, i32>("special_defense"));
        pokemon_attribs.speed.push(row.get::<_, i32>("speed"));
        pokemon_attribs
            .types
            .push(row.get::<_, Option<String>>("types").unwrap_or_default());
    }

    // Creating the DataFrame with 'Polars'
    let df = DataFrame::new(vec![
        Series::new("id".into(), pokemon_attribs.id),
        Series::new("random_id".into(), pokemon_attribs.random_id),
        Series::new("pokedex_number".into(), pokemon_attribs.pokedex_number),
        Series::new("name".into(), pokemon_attribs.name),
        Series::new("height".into(), pokemon_attribs.height),
        Series::new("weight".into(), pokemon_attribs.weight),
        Series::new("hp".into(), pokemon_attribs.hp),
        Series::new("attack".into(), pokemon_attribs.attack),
        Series::new("defense".into(), pokemon_attribs.defense),
        Series::new("special_attack".into(), pokemon_attribs.special_attack),
        Series::new("special_defense".into(), pokemon_attribs.special_defense),
        Series::new("speed".into(), pokemon_attribs.speed),
        Series::new("types".into(), pokemon_attribs.types),
    ])?;

    plot_distributions(&df.clone())?;
    plot_type_combinations(&df.clone())?;

    Ok(df)
}

// Plotting the distributions in a PNG image
fn plot_distributions(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    // Creating the drawing area (Canvas)
    let root = BitMapBackend::new("pokemon_distribution.png", (1600, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Split the canvas in a 2x4 grid
    let areas = root.split_evenly((2, 4));
    // List of columns to plot
    let columns = vec![
        "weight",
        "height",
        "attack",
        "speed",
        "hp",
        "special_attack",
        "special_defense",
        "defense",
    ];

    println!("DataFrame shape: {:?}", df.shape());
    // Iterating over each area and column
    for (area, &column) in areas.into_iter().zip(columns.iter()) {
        // Getting the series for the current column
        let series = df.column(column)?;

        // Casting the series types (Strings and integers) into 64-bits float
        // If the type is f64 just clone the series
        let f64_series: Result<Series, PolarsError> = match series.dtype() {
            DataType::Float64 => Ok(series.clone()),
            DataType::Int32 => series.cast(&DataType::Float64),
            DataType::String => {
                // For string columns, try to parse as f64
                let parsed: Vec<Option<f64>> = series
                    .str()?
                    .into_iter()
                    .map(|opt_s| opt_s.and_then(|s| s.parse::<f64>().ok()))
                    .collect();
                Ok(Series::new(column.into(), parsed))
            }
            _ => {
                println!("Unsupported data type for column: {column}. Skipping");
                continue;
            }
        };

        // Handling potential errors in the string casting
        let f64_series = match f64_series {
            Ok(s) => s,
            Err(e) => {
                println!("Error converting column {column} to f64: {e}. Skipping");
                continue;
            }
        };

        // Getting the f64 chunked array
        let f64_chunked = match f64_series.f64() {
            Ok(chunk) => chunk,
            Err(e) => {
                println!("Error getting f64 chunk for column {column}: {e}. Skipping");
                continue;
            }
        };

        // Handling the empty chunked array
        if f64_chunked.is_empty() {
            // If the columns are empty, the pokedex skip the column
            println!("Column: {column} is empty after conversion. Skipping");
            continue;
        }

        let min = f64_chunked.min().unwrap_or(0.0);
        let max = f64_chunked.max().unwrap_or(0.0);
        // Skipping no variant columns
        if (max - min).abs() < f64::EPSILON {
            println!("Column: {column} has no variation. Skipping");
            continue;
        }

        let range = max - min;
        let bin_count = 20;
        let bin_size = range / bin_count as f64;

        // Creating bins and counter values in each bin
        let mut bins = vec![0; bin_count];
        for value in f64_chunked.into_iter().flatten() {
            let bin = ((value - min) / bin_size).floor() as usize;
            if bin < bin_count {
                bins[bin] += 1;
            }
        }

        // Getting the max bin count
        let y_max = *bins.iter().max().unwrap_or(&0) as f64;
        if y_max == 0.0 {
            println!("Column: {column} has no non-zero bins. Skipping");
            continue;
        }

        // Creating a new chart for the canvas
        let mut chart = ChartBuilder::on(&area)
            .margin(5)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 30)
            .caption(format!("Distribution of {}", column), ("sans-serif", 20))
            .build_cartesian_2d((min..max).step(bin_size), 0f64..(y_max * 1.1))?;

        chart.configure_mesh().draw()?;

        println!("Bins for {column}: {:?}", bins);
        // Drawing the histogram
        chart.draw_series(
            Histogram::vertical(&chart).style(BLUE.filled()).data(
                bins.iter()
                    .enumerate()
                    .map(|(i, &count)| (min + i as f64 * bin_size, count as f64)),
            ),
        )?;
    }

    root.present()?;

    Ok(())
}

fn plot_type_combinations(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("type_combinations.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    Ok(())
}
