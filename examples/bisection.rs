use rmath::vector;

#[rustfmt::skip]
fn main() {
    fn f(x: f64) -> f64 {
        x.ln() - (-x).exp()
    }
    let a= bisection(&BisectionParams { x_left: 1., x_right: 2., tolerance: 1e-7, }, f);
    vector::all_eq([a], [1.3098], &vector::AllEqParams { tolerance: 1e-3, scale: vector::all_eq_no_scale() });
}

#[derive(Debug, Clone, Copy)]
pub struct BisectionParams {
    pub x_left: f64,
    pub x_right: f64,
    pub tolerance: f64,
}
#[rustfmt::skip]
pub fn bisection(params: &BisectionParams, f: impl Fn(f64) -> f64) -> f64 {
    let f_l = f(params.x_left);
    let f_r = f(params.x_right);
    if f_l == 0. {
        return params.x_left;
    }
    if f_r == 0. {
        return params.x_right;
    }
    assert!(f_l.is_sign_positive() != f_r.is_sign_positive());
    let mut a = params.x_left;
    let mut b = params.x_right;
    loop {
        let m = a / 2. + b / 2.;
        if vector::all_eq([a], [b], &vector::AllEqParams { tolerance: params.tolerance, scale: vector::all_eq_no_scale() }) {
            return m;
        }
        let f_a = f(a);
        let f_m = f(m);
        if f_m == 0. {
            return m;
        }
        if f_a.is_sign_positive() != f_m.is_sign_positive() {
            b = m;
        } else {
            a = m;
        }
    }
}
