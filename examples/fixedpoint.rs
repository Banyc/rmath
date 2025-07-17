use rmath::vector;

#[rustfmt::skip]
fn main() {
    // find root for f(x) = log(x) - exp(-x) = 0
    let assert = |a: Option<f64>| assert!(vector::all_eq([a.unwrap()], [1.3098], &vector::AllEqParams { tolerance: 10e-7, scale: vector::all_eq_no_scale() }));
    let a = fixed_point(|x| (-x).exp().exp(), &FixedPointParams { start: 2., tolerance: 10e-7, max_iters: 100 });
    assert(a);
    let a = fixed_point(|x| x - x.ln() + (-x).exp(), &FixedPointParams { start: 2., tolerance: 10e-7, max_iters: 100 });
    assert(a);
    let a = fixed_point(|x| x.ln() - (-x).exp() + x, &FixedPointParams { start: 2., tolerance: 10e-7, max_iters: 100 });
    assert!(a.is_none());
}

pub struct FixedPointParams {
    pub start: f64,
    pub tolerance: f64,
    pub max_iters: usize,
}
#[rustfmt::skip]
pub fn fixed_point(f: impl Fn(f64) -> f64, params: &FixedPointParams) -> Option<f64> {
    let mut x = params.start;
    for _ in 0..params.max_iters {
        let y = f(x);
        if vector::all_eq([x], [y], &vector::AllEqParams { tolerance: params.tolerance, scale: vector::all_eq_no_scale() }) {
            return Some(y);
        }
        x = y
    }
    None
}
