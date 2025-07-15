use rmath::vector;

fn main() {
    fn f(x: f64) -> (f64, f64) {
        (x.ln() - (-x).exp(), 1. / x + (-x).exp())
    }
    let a = newtonraphson(
        &NewtonRaphsonParams {
            start: 2.,
            tolerance: 1e-6,
            iter: 100,
        },
        f,
    );
    dbg!(a);
    #[rustfmt::skip]
    assert!(vector::all_eq([a.unwrap()], [1.3098], &vector::AllEqParams { tolerance: 1e-3, scale: vector::all_eq_no_scale() }));
}

#[derive(Debug, Clone, Copy)]
pub struct NewtonRaphsonParams {
    pub start: f64,
    pub tolerance: f64,
    pub iter: usize,
}
pub fn newtonraphson(params: &NewtonRaphsonParams, f: impl Fn(f64) -> (f64, f64)) -> Option<f64> {
    let mut x = params.start;
    for _ in 0..params.iter {
        let (y, d) = f(x);
        let close_enough = y.abs() < params.tolerance;
        if close_enough {
            return Some(x);
        }
        x -= y / d;
    }
    None
}
