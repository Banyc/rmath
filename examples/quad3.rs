use rmath::{
    plot::{PlotType, plot},
    vector::{self, SeqParams, seq},
};

fn main() {
    let x = seq(SeqParams {
        start: -2.,
        end: 2.,
        step: 0.01,
    });
    let c = QuadConsts {
        a: 1.,
        b: 0.,
        c: -1.,
    };
    let y = vector::map(&x, |x| quad3(&c)(x));
    println!("roots: {:?}", quad3_root(&c));
    plot(&x, &y, PlotType::Line, None);
}

pub struct QuadConsts {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
fn quad3(c: &QuadConsts) -> impl Fn(f64) -> f64 + Copy {
    move |x| c.a * x.powi(2) + c.b * x + c.c
}
fn quad3_root(c: &QuadConsts) -> Vec<f64> {
    let discriminant = c.b.powi(2) - 4. * c.a * c.c;
    let d_sqrt = discriminant.sqrt();
    vector::div(vector::add([-c.b], [d_sqrt, -d_sqrt]), [2. * c.a])
}
