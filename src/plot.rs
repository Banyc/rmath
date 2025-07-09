pub struct Plot {
    plot: plotly::Plot,
}
impl std::fmt::Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>()).finish()
    }
}
impl Default for Plot {
    fn default() -> Self {
        Self::new()
    }
}
impl Plot {
    pub fn new() -> Plot {
        Self {
            plot: plotly::Plot::new(),
        }
    }
    pub fn show(&self) {
        self.plot.show();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlotType {
    Point,
    Line,
    PointOnLine,
}
pub fn plot<X, Y>(x: impl AsRef<[X]>, y: impl AsRef<[Y]>, ty: PlotType, plot: Option<&mut Plot>)
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
    add_trace_and_opt_show(trace, plot);
}

fn add_trace_and_opt_show(trace: Box<dyn plotly::Trace>, plot: Option<&mut Plot>) {
    let show = plot.is_none();
    let plot = match plot {
        Some(plot) => &mut plot.plot,
        None => &mut plotly::Plot::new(),
    };
    plot.add_trace(trace);
    if show {
        plot.show();
    }
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
        plot(x, y, PlotType::Point, None);
        let mut p = Plot::new();
        plot(x, y, PlotType::Point, Some(&mut p));
    }
}
