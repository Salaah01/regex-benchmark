use std::time::Duration;

use plotters::{coord::types::RangedCoordf32, prelude::*};

use crate::{calc_regex_speed::SpeedTestResult, enums::TimeUnit};

fn duration_repr(duration: Duration, units: &TimeUnit) -> i32 {
    match units {
        TimeUnit::Nanoseconds => duration.as_nanos() as i32,
        TimeUnit::Microseconds => duration.as_micros() as i32,
        TimeUnit::Milliseconds => duration.as_millis() as i32,
        TimeUnit::Seconds => duration.as_secs() as i32,
    }
}

pub fn create(
    results: Vec<SpeedTestResult>,
    max_x: i32,
    max_y: i32,
    units: &TimeUnit,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("graph.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // let mut chart = ChartBuilder::on(&root)
    //     .caption("Big O Complexity", ("sans-serif", 32).into_font())
    //     .margin(5)
    //     .x_label_area_size(30)
    //     .y_label_area_size(30)
    //     .build_cartesian_2d(0..max_x, 0..max_y)?;

    // chart.configure_mesh().draw()?;
    // chart.draw_series(LineSeries::new(
    //     results
    //         .iter()
    //         .map(|result| (result.length as i32, duration_repr(result.duration, units))),
    //     &RED,
    // ))?;

    let areas = root.split_by_breakpoints([944], [80]);
    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;
    scatter_ctx.draw_series(
        results
            .iter()
            .map(|res| Circle::new((res.length, res.duration), 2, GREEN.filled())),
    )?;
    Ok(())
}
