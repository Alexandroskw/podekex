use plotters::prelude::*;
use polars::prelude::*;
use postgres::Client;
use std::collections::HashMap;
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
    correlation_analysis(&df.clone())?;

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
    let root = BitMapBackend::new("type_combinations.png", (2000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    // Fetching the colum "types" from the DataFrame
    let types_column = df.column("types")?.str()?;
    let mut type_counts = HashMap::new();

    // Itreating in all the types in the column
    for type_str in types_column.into_iter().flatten() {
        // Searching for a separator in the JSON
        for type_name in type_str.split(", ") {
            *type_counts.entry(type_name.to_string()).or_insert(0u32) += 1;
        }
    }

    // Casting the HashMap to vector and sorting from largest to smallest and taken the first 20
    let mut type_count_vec: Vec<_> = type_counts.into_iter().collect();
    type_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    type_count_vec.truncate(20);

    // Separating the names of the types and their counts in different vectors
    let type_names: Vec<String> = type_count_vec
        .iter()
        .map(|(name, _)| name.clone())
        .collect();
    let counts: Vec<u32> = type_count_vec.iter().map(|(_, count)| *count).collect();

    let max_count = *counts.iter().max().unwrap_or(&0);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(100)
        .margin(5)
        .caption("Top 20 Type Combinations", ("sans-serif", 50.0))
        .build_cartesian_2d(0..max_count, 0..type_names.len())?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_desc("Type Combination")
        .x_desc("Count")
        .axis_desc_style(("sans-serif", 30))
        .y_labels(type_names.len())
        .draw()?;

    // Drawing the bars
    chart.draw_series(type_names.iter().zip(counts.iter()).enumerate().map(
        |(i, (_, &count))| {
            let mut bar = Rectangle::new([(0, i), (count, i + 1)], BLUE.filled());
            bar.set_margin(0, 0, 1, 1);
            bar
        },
    ))?;

    // Drawing the text labels
    chart.draw_series(type_names.iter().enumerate().map(|(i, name)| {
        Text::new(
            name.to_string(),
            (0, i),
            ("sans-serif", 20).into_font().color(&BLACK),
        )
    }))?;

    // Adding the values at the end of each bar (this section is optional)
    // chart.draw_series(counts.iter().enumerate().map(|(i, &count)| {
    //     Text::new(
    //         count.to_string(),
    //         (count, i),
    //         ("sans-serif", 20).into_font().color(&BLACK),
    //     )
    // }))?;

    root.present()?;

    println!("Chart has been saved to type_combinations.png");
    Ok(())
}

fn correlation_analysis(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    // Fetching the hp and height and weight columns
    let corr_hp = df.columns(["hp", "height", "weight"])?;

    let hp = corr_hp[0].i32()?;
    let height = corr_hp[1].f64()?;
    let weight = corr_hp[2].f64()?;

    // Calculating the middle of the columns
    let hp_mean = hp.mean().unwrap();
    let height_mean = height.mean().unwrap();
    let weight_mean = weight.mean().unwrap();

    let mut covariance: f64 = 0.0;
    let mut hp_variance: f64 = 0.0;
    let mut h_variance: f64 = 0.0;
    let mut w_variance: f64 = 0.0;

    for (h, ht) in hp.into_no_null_iter().zip(height.into_no_null_iter()) {
        let h_f64 = h as f64;
        covariance += (h_f64 - hp_mean) * (ht - height_mean);
        hp_variance += (h_f64 - hp_mean).powi(2);
        h_variance += (ht - hp_mean).powi(2);
    }

    let correlation = covariance / (hp_variance.sqrt() * h_variance.sqrt());
    println!("Correlation {correlation}");

    Ok(())
}
