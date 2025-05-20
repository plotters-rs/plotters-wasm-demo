use crate::DrawResult;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

/// Draw power function f(x) = x^power.
pub fn draw(
    canvas_id: String,
    power: i32,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f64, f64)>> {
    let backend = CanvasBackend::new(canvas_id.as_str()).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=x^{}", power), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(-1f64..1f64, -1.2f64..1.2f64)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    chart.draw_series(LineSeries::new(
        (-50..=50)
            .map(|x| x as f64 / 50.0)
            .map(|x| (x, x.powf(power as f64))),
        &RED,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}

