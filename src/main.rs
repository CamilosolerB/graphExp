use plotters::prelude::*;
use std::f64::consts::E;

fn int_identidad(b: f64) -> f64 {
    return E.powf(b);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (1024, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let b = 10.0;
    let iterations = 100.0;
    let slots = b/iterations;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..50f64, 0f64..1500f64)?;

    chart
        .configure_mesh()
        .draw()?;

    let x: Vec<f64> = (0..=iterations as i32).map(|i| i as f64 * slots).collect();
    let y: Vec<f64> = x.iter().map(|&i| int_identidad(i)).collect();

    for (i, (&x_val, &y_val)) in x.iter().zip(y.iter()).enumerate() {
        println!(" x: {} | f(x) {} | x - f(x) {}",x_val, y_val, (x_val - y_val))
        //println!("Punto {}: ({}, {}), diferencia con la integral = {}", i, x_val, y_val,(int_identidad(i as f64) - y_val));
    }
    chart.draw_series(LineSeries::new(
        x.iter().zip(y.iter()).map(|(&x, &y)| (x, y)),
        &RED,
    ))?;

    Ok(())
}
