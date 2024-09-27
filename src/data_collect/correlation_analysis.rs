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

    // Creating the plot
    let root = BitMapBackend::new("hp_correlation", (1200, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let areas = root.split_evenly((2, 1));
    let columns = ["height", "weight"];
    for (area, &columns) in areas.into_iter().zip(columns.iter()) {
        let series = df.column(columns)?;
        let f64_series: Result<Series, PolarsError> = match series.dtype() {
            DataType::Float64 => Ok(series.clone()),
            DataType::Int32 => series.cast(&DataType::Float64),
            DataType::String => {
                let parsed: Vec<Option<f64>> = series
                    .str()?
                    .into_iter()
                    .map(|opt| opt.and_then(|s| s.parse::<f64>().ok()))
                    .collect();
                Ok(Series::new(columns.into(), parsed))
            }
            _ => {
                println!("Unsupported data type for column: {columns}. Skipping");
                continue;
            }
        };
    }

    Ok(())
}
