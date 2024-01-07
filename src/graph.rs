use std::time::Duration;

use plotters::prelude::*;

use crate::{calc_regex_speed::SpeedTestResult, enums::TimeUnit};

fn duration_repr(duration: Duration, units: &TimeUnit) -> i32 {
    match units {
        TimeUnit::Nanoseconds => duration.as_nanos() as i32,
        TimeUnit::Microseconds => duration.as_micros() as i32,
        TimeUnit::Milliseconds => duration.as_millis() as i32,
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

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..max_x, 0..max_y)?;
    scatter_ctx.configure_mesh().draw()?;

    scatter_ctx
        .draw_series(results.iter().map(|res| {
            let x = res.length as i32;
            let y = duration_repr(res.duration, units);
            let color = RED.mix(0.2);
            let size = 2;
            println!("({}, {})", x, y);
            Circle::new((x, y), size, color.filled())
        }))
        .unwrap();

    Ok(())
}
