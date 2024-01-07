//! Graphing module

use plotters::prelude::*;

use crate::{calc_regex_speed::SpeedTestResult, duration_utils::duration_repr, enums::TimeUnit};

/// Creates the graph for the given results.
///
/// Arguments:
/// * `results` - The results of the speed tests.
/// * `max_x` - The maximum x value for the x-axis.
/// * `min_y` - The minimum y value for the y-axis.
/// * `max_y` - The maximum y value for the y-axis.
/// * `units` - The time unit to be used for the y-axis and the data itself.
pub fn create(
    results: Vec<SpeedTestResult>,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    units: &TimeUnit,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("graph.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut graph = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(20)
        .caption("Regex Benchmark", ("sans-serif", 40).into_font())
        .build_cartesian_2d(0..max_x, min_y..max_y)?;
    graph
        .configure_mesh()
        .x_desc("String Length")
        .y_desc(format!("Time in {:?} ({})", units, units))
        .draw()?;

    graph
        .draw_series(results.iter().map(|res| {
            let x = res.length as i32;
            let y = duration_repr(res.duration, units) as i32;
            let color = RED.mix(0.2);
            let size = 2;
            Circle::new((x, y), size, color.filled())
        }))
        .unwrap();

    Ok(())
}
