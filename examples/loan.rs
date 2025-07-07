fn main() {
    let n = years_pay_off_loan(&LoanParams {
        r: 0.11,
        period: 1. / 12.,
        debt_initial: 1000.,
        repayments: 12.,
    });
    println!("{n}");
}

#[derive(Debug, Clone, Copy)]
pub struct LoanParams {
    /// annual interest rate
    pub r: f64,
    /// years between repayments
    pub period: f64,
    /// amount borrowed
    pub debt_initial: f64,
    /// amount repaid each period
    pub repayments: f64,
}
#[rustfmt::skip]
fn years_pay_off_loan(params: &LoanParams) -> f64 {
    let mut debt = params.debt_initial;
    let mut n = 0.;
    while 0. < debt {
        debt = debt * (1. + params.r * params.period) - params.repayments;
        n += params.period;
    }
    n
}
