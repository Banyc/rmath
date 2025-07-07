pub fn plot<X, Y>(x: impl AsRef<[X]>, y: impl AsRef<[Y]>, ty: PlotType)
where
    X: Clone + serde::Serialize + 'static,
    Y: Clone + serde::Serialize + 'static,
{
    let x = x.as_ref();
    let y = y.as_ref();
    assert_eq!(x.len(), y.len());
    let mode = match ty {
        PlotType::Point => plotly::common::Mode::Markers,
        PlotType::Line => plotly::common::Mode::Lines,
        PlotType::PointOnLine => plotly::common::Mode::LinesMarkers,
    };
    let trace = plotly::Scatter::new(x.to_vec(), y.to_vec()).mode(mode);
    let mut plot = plotly::Plot::new();
    plot.add_trace(trace);
    let layout = plotly::Layout::new();
    plot.set_layout(layout);
    plot.show();
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlotType {
    Point,
    Line,
    PointOnLine,
}

#[cfg(test)]
mod tests {
    use crate::vector::{SeqParams, add, cast, div, pow, seq};

    use super::*;

    #[test]
    #[ignore]
    fn exponential_limit() {
        #[rustfmt::skip]
        let x = &seq(SeqParams {
            start: 10, end: 200, step: 10 });
        let x = &cast::<_, f64>(x);
        let y = &pow(add([1.], div([1.], x)), x);
        plot(x, y, PlotType::Point);
    }
}
