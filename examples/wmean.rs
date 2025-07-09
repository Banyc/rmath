use rmath::{property::IsOrd, vector};

fn main() {
    let x = &[
        8.244, 51.421, 39.020, 90.574, 44.697, 83.600, 73.760, 81.106, 38.811, 68.517,
    ];
    let wmean = wmean(x, 2);
    #[rustfmt::skip]
    assert!(vector::all_eq([wmean], [59.8773], &vector::AllEqParams { tolerance: 10e-7, scale: vector::all_eq_no_scale() }));
}

fn wmean<T>(x: impl AsRef<[T]>, k: usize) -> T
where
    T: IsOrd + Copy + PartialOrd + num_traits::Float + num_traits::FromPrimitive + std::fmt::Debug,
{
    let x = x.as_ref();
    let mut x = vector::sort(x);
    let lpos = k;
    let rpos = x.len() - k - 1;
    let a = x[lpos];
    let b = x[rpos];
    vector::set(&mut x, ..lpos, [a], || panic!());
    vector::set(&mut x, rpos + 1.., [b], || panic!());
    vector::mean(x)
}
