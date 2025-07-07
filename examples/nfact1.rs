use rmath::vector::{self, SeqParams, seq};

#[rustfmt::skip]
fn main() {
    let a = seq(SeqParams::from(1..=6));
    let n_factorial = vector::prod(a);
    println!("{n_factorial}");
    assert_eq!(n_factorial, 720);
}
