use rmath::{
    plot::{PlotType, plot},
    vector::{self, SeqParams, seq},
};

fn main() {
    pension(&PensionParams {
        r: 0.11,
        term: 10.,
        period: 1. / 12.,
        payments: 100.,
    });
}

#[derive(Debug, Clone, Copy)]
pub struct PensionParams {
    /// annual interest rate
    pub r: f64,
    /// forecast years
    pub term: f64,
    /// years between payments
    pub period: f64,
    /// amount deposited each period
    pub payments: f64,
}
#[rustfmt::skip]
fn pension(params: &PensionParams) {
    let n = (params.term / params.period).floor() as usize; // number of payments
    let mut pension = vec![0.];
    for _ in 0..n {
        let prev_pension = pension.last().unwrap();
        pension.push(*prev_pension * (1. + params.r * params.period) + params.payments);
    }
    let time = vector::mul(
        vector::cast::<_, f64>(seq(SeqParams::from(0..=n))),
        [params.period]);
    plot(time, pension, PlotType::Point);
}
