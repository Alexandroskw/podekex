use plotters::prelude::*;
use polars::prelude::*;

pub fn correlation_analysis(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    // Fetching the hp and height and weight columns
    let corr_hp = df.columns(["hp", "height", "weight"])?;

    let hp = corr_hp[0].i32()?;
    let height = corr_hp[1].str()?;
    let weight = corr_hp[2].str()?;

    let height_f64: Vec<f64> = height
        .into_iter()
        .filter_map(|h| h.and_then(|s| s.parse::<f64>().ok()))
        .collect();
    let weight_f64: Vec<f64> = weight
        .into_iter()
        .filter_map(|w| w.and_then(|s| s.parse::<f64>().ok()))
        .collect();

    // Calculating the middle of the columns
    let hp_mean = hp.mean().unwrap();
    let height_mean = height_f64.iter().sum::<f64>() / height_f64.len() as f64;
    let weight_mean = weight_f64.iter().sum::<f64>() / weight_f64.len() as f64;

    // Initialize the covariance and variance at 0
    let mut covariance_hp_height: f64 = 0.0;
    let mut covariance_hp_weight: f64 = 0.0;
    let mut hp_variance: f64 = 0.0;
    let mut h_variance: f64 = 0.0;
    let mut w_variance: f64 = 0.0;

    for ((h, ht), w) in hp
        .into_iter()
        .zip(height_f64.into_iter())
        .zip(weight_f64.into_iter())
    {
        if let Some(h) = h {
            let h_f64 = h as f64;
            covariance_hp_height += (h_f64 - hp_mean) * (ht - height_mean);
            covariance_hp_weight += (h_f64 - hp_mean) * (w - weight_mean);
            hp_variance += (h_f64 - hp_mean).powi(2);
            h_variance += (ht - height_mean).powi(2);
            w_variance += (w - weight_mean).powi(2);
        }
    }

    let correlation_hp_height = covariance_hp_height / (hp_variance.sqrt() * h_variance.sqrt());
    let correlation_hp_weight = covariance_hp_weight / (hp_variance.sqrt() * w_variance.sqrt());
    println!(
        "Correlations
            hp vs height: {correlation_hp_height}
            hp vs weight: {correlation_hp_weight}"
    );
    plot_scatter(df)?;

    Ok(())
}

fn plot_scatter(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("pokemon_scatter_plots.png", (1600, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let areas = root.split_evenly((1, 2));
    let columns = ["height", "weight"];

    let hp_series = df.column("hp")?.i32()?;

    for (area, &column) in areas.into_iter().zip(columns.iter()) {
        let series = df.column(column)?;

        let f64_series: Vec<f64> = match series.dtype() {
            DataType::Float64 => series.f64()?.into_iter().flatten().collect(),
            DataType::Int32 => series
                .i32()?
                .into_iter()
                .map(|v| v.map(|i| i as f64))
                .flatten()
                .collect(),
            DataType::String => series
                .str()?
                .into_iter()
                .filter_map(|opt_s| opt_s.and_then(|s| s.parse::<f64>().ok()))
                .collect(),
            _ => {
                println!("Unsupported data type for column: {column}. Skipping");
                continue;
            }
        };

        if f64_series.is_empty() {
            println!("Column: {column} is empty after conversion. Skipping");
            continue;
        }

        let x_min = f64_series.iter().cloned().fold(f64::INFINITY, f64::min);
        let x_max = f64_series.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let y_min = hp_series.min().unwrap_or(0) as f64;
        let y_max = hp_series.max().unwrap_or(255) as f64;

        let mut chart = ChartBuilder::on(&area)
            .margin(5)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption(format!("HP vs {}", column), ("sans-serif", 40))
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(
                hp_series
                    .into_iter()
                    .zip(f64_series.iter())
                    .filter_map(|(hp, &x)| hp.map(|hp| Circle::new((x, hp as f64), 2, BLUE))),
            )?
            .label("Pokémon")
            .legend(|(x, y)| Circle::new((x, y), 3, BLUE));

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;
    }

    root.present()?;

    println!("Scatter plots have been saved to pokemon_scatter_plots.png");
    Ok(())
}
