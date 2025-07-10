use rmath::{
    plot::{PlotType, plot},
    vector,
};

fn main() {
    let n = 1000;
    let x = vector::seq(vector::SeqParams::from(2..=n));
    let is_prime = vector::map(&x, is_prime);
    let prime = vector::cumsum(vector::map(is_prime, |x| x as usize));
    let prime_density = vector::div(vector::cast::<_, f64>(prime), vector::cast::<_, f64>(&x));
    plot(&x, &prime_density, PlotType::Line, None);
}

fn is_prime(n: usize) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    for m in 2..=(n / 2) {
        if n.is_multiple_of(m) {
            return false;
        }
    }
    true
}
