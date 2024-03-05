use plotters::prelude::*;
use std::f64::consts::E;

fn int_identidad(b: f64) -> f64 {
    return E.powf(b);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (1024, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..50f64, 0f64..150f64)?;

    chart
        .configure_mesh()
        .draw()?;

    let x: Vec<f64> = (0..=1000).map(|i| i as f64 * 0.1).collect();
    let y: Vec<f64> = x.iter().map(|&i| int_identidad(i)).collect();

    for (i, (&x_val, &y_val)) in x.iter().zip(y.iter()).enumerate() {
        println!("Punto {}: ({}, {}), diferencia con la integral = {}", i, x_val, y_val,(E.powf(i as f64) - y_val));
    }
    chart.draw_series(LineSeries::new(
        x.iter().zip(y.iter()).map(|(&x, &y)| (x, y)),
        &RED,
    ))?;

    Ok(())
}
