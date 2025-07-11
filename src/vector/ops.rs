use std::{
    collections::HashMap,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use num_traits::{One, Pow, Zero};

use crate::property::IsOrd;

pub fn cast<A, B>(a: impl AsRef<[A]>) -> Vec<B>
where
    A: num_traits::AsPrimitive<B>,
    B: Copy + 'static,
{
    a.as_ref().iter().map(|a| (*a).as_()).collect()
}

#[rustfmt::skip]
pub fn pmin<T, Slice>(vectors: impl AsRef<[Slice]>) -> Vec<T>
where T: PartialOrd + IsOrd + Clone, Slice: AsRef<[T]> {
    vectors.as_ref().iter().map(|vector| min(vector.as_ref())).collect() }
#[rustfmt::skip]
pub fn pmax<T, Slice>(vectors: impl AsRef<[Slice]>) -> Vec<T>
where T: PartialOrd + IsOrd + Clone, Slice: AsRef<[T]> {
    vectors.as_ref().iter().map(|vector| max(vector.as_ref())).collect() }

#[rustfmt::skip]
pub fn sum<T>(vector: impl AsRef<[T]>) -> T where T: Clone + AddAssign + Zero {
    vector.as_ref().iter().fold(T::zero(), |mut cum, a| {
        cum += a.clone(); cum }) }
#[rustfmt::skip]
pub fn prod<T>(vector: impl AsRef<[T]>) -> T where T: Clone + MulAssign + One {
    vector.as_ref().iter().fold(T::one(), |mut cum, a| {
        cum *= a.clone(); cum }) }
#[rustfmt::skip]
pub fn min<T>(vector: impl AsRef<[T]>) -> T where T: PartialOrd + IsOrd + Clone {
    find_ord_one_by(vector, |a, b| a < b) }
#[rustfmt::skip]
pub fn max<T>(vector: impl AsRef<[T]>) -> T where T: PartialOrd + IsOrd + Clone {
    find_ord_one_by(vector, |a, b| b < a) }
fn find_ord_one_by<T>(vector: impl AsRef<[T]>, choose_left: impl Fn(T, T) -> bool) -> T
where
    T: IsOrd + Clone,
{
    let mut curr_choice = None;
    for item in vector.as_ref() {
        let true = item.is_ord() else {
            // continue;
            return item.clone();
        };
        let Some(prev_choice) = curr_choice else {
            curr_choice = Some(item);
            continue;
        };
        if choose_left(item.clone(), prev_choice.clone()) {
            curr_choice = Some(item);
        }
    }
    curr_choice.unwrap().clone()
}
pub fn sort<T>(vector: impl AsRef<[T]>) -> Vec<T>
where
    T: IsOrd + Clone + PartialOrd,
{
    let vector = vector.as_ref();
    let mut out = vec![];
    if T::always_ord() {
        out = vector.to_vec();
    } else {
        for item in vector {
            if item.is_ord() {
                out.push(item.clone());
            }
        }
    }
    out.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    out
}
pub fn mean<T>(vector: impl AsRef<[T]>) -> T
where
    T: num_traits::Float + num_traits::FromPrimitive,
{
    let vector = vector.as_ref();
    assert!(!vector.is_empty());
    let mut partial_mean = T::zero();
    let len = vector.len();
    for item in vector {
        let a = item.div(T::from_usize(len).unwrap());
        partial_mean = partial_mean.add(a);
    }
    partial_mean
}
pub fn var<T>(vector: impl AsRef<[T]>) -> T
where
    T: num_traits::Float + num_traits::FromPrimitive,
{
    let vector = vector.as_ref();
    if vector.len() == 1 {
        return T::nan();
    }
    let mut partial_var = T::zero();
    let len = vector.len();
    let mean = mean(vector);
    for item in vector {
        let a = item.sub(mean).powi(2);
        let a = a.div(T::from_usize(len - 1).unwrap());
        partial_var = partial_var.add(a);
    }
    partial_var
}
pub fn any(vector: impl AsRef<[bool]>) -> bool {
    vector.as_ref().iter().any(|x| *x)
}
pub fn all(vector: impl AsRef<[bool]>) -> bool {
    vector.as_ref().iter().all(|x| *x)
}

pub fn cumsum<T>(vector: impl AsRef<[T]>) -> Vec<T>
where
    T: std::ops::AddAssign + num_traits::Zero + Clone,
{
    let mut out = vec![];
    let mut sum = T::zero();
    for item in vector.as_ref() {
        sum += item.clone();
        out.push(sum.clone());
    }
    out
}

pub use single_vector_in_single_vector_out::*;
#[rustfmt::skip]
mod single_vector_in_single_vector_out {
    use crate::property::CalcFactorial;
    pub fn neg<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: std::ops::Neg<Output = T> + Clone {
        vector.as_ref().iter().map(|x| x.clone().neg()).collect() }
    pub fn not(vector: impl AsRef<[bool]>) -> Vec<bool> {
        vector.as_ref().iter().map(|x| !x).collect() }
    pub fn exp<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: num_traits::Float {
        vector.as_ref().iter().map(|x| x.exp()).collect() }
    pub fn sqrt<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: num_traits::Float {
        vector.as_ref().iter().map(|x| x.sqrt()).collect() }
    pub fn abs<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: num_traits::Signed {
        vector.as_ref().iter().map(|x| x.abs()).collect() }
    pub fn sin<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: num_traits::Float {
        vector.as_ref().iter().map(|x| x.sin()).collect() }
    pub fn cos<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: num_traits::Float {
        vector.as_ref().iter().map(|x| x.cos()).collect() }
    pub fn tan<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: num_traits::Float {
        vector.as_ref().iter().map(|x| x.tan()).collect() }
    pub fn is_nan<T>(vector: impl AsRef<[T]>) -> Vec<bool> where T: num_traits::Float {
        vector.as_ref().iter().map(|x| x.is_nan()).collect() }
    pub fn rm_nan<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: num_traits::Float {
        vector.as_ref().iter().copied().filter(|x| !x.is_nan()).collect() }
    pub fn floor<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: num_traits::Float {
        vector.as_ref().iter().map(|x| x.floor()).collect() }
    pub fn ceil<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: num_traits::Float {
        vector.as_ref().iter().map(|x| x.ceil()).collect() }
    pub fn round<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: num_traits::Float {
        vector.as_ref().iter().map(|x| x.round()).collect() }
    pub fn factorial<T>(vector: impl AsRef<[T]>) -> Vec<T> where T: CalcFactorial {
        vector.as_ref().iter().map(|x| x.factorial()).collect() }
}
pub use two_vectors_in_single_vector_out::*;
#[rustfmt::skip]
mod two_vectors_in_single_vector_out {
    use crate::property::CalcChoose;
    use super::*;
    pub fn add<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T> where T: Clone + AddAssign {
        circle_zip(a, b, |mut a, b| { a += b; a }) }
    pub fn sub<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T> where T: Clone + SubAssign {
        circle_zip(a, b, |mut a, b| { a -= b; a }) }
    pub fn mul<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T> where T: Clone + MulAssign {
        circle_zip(a, b, |mut a, b| { a *= b; a }) }
    pub fn div<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T> where T: Clone + DivAssign {
        circle_zip(a, b, |mut a, b| { a /= b; a }) }
    pub fn modulo<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T> where T: Clone + num_traits::PrimInt {
        circle_zip(a, b, |a, b| a % b) }
    pub fn pow<A, B>(a: impl AsRef<[A]>, b: impl AsRef<[B]>) -> Vec<A> where A: Clone + Pow<B, Output = A>, B: Clone {
        circle_zip(a, b, |a, b| a.pow(b)) }
    pub fn lt<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<bool> where T: PartialOrd + Clone {
        circle_zip(a, b, |a, b| a < b) }
    pub fn gt<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<bool> where T: PartialOrd + Clone {
        circle_zip(a, b, |a, b| a > b) }
    pub fn eq<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<bool> where T: PartialEq + Clone {
        circle_zip(a, b, |a, b| a == b) }
    pub fn neq<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<bool> where T: PartialEq + Clone {
        circle_zip(a, b, |a, b| a != b) }
    pub fn or(a: impl AsRef<[bool]>, b: impl AsRef<[bool]>) -> Vec<bool> {
        circle_zip(a, b, |a, b| a || b) }
    pub fn and(a: impl AsRef<[bool]>, b: impl AsRef<[bool]>) -> Vec<bool> {
        circle_zip(a, b, |a, b| a && b) }
    pub fn xor(a: impl AsRef<[bool]>, b: impl AsRef<[bool]>) -> Vec<bool> {
        circle_zip(a, b, |a, b| a ^ b) }
    pub fn choose<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T> where T: CalcChoose + Clone {
        circle_zip(a, b, |a, b| a.choose(b)) }
}

#[rustfmt::skip]
pub fn map<A, B>(vector: impl AsRef<[A]>, fmap: impl Fn(A) -> B) -> Vec<B> where A: Clone {
    vector.as_ref().iter().map(|x| fmap(x.clone())).collect() }

/// docs: <https://web.mit.edu/r/current/lib/R/library/base/html/all.equal.html>
/// src: <https://github.com/wch/r-source/blob/67e3ab91b0489f56520142ce9352d68aa9a49ab0/src/library/base/R/all.equal.R#L100>
#[rustfmt::skip]
pub fn all_eq<T>(
    a: impl AsRef<[T]>,
    b: impl AsRef<[T]>,
    params: &AllEqParams<T, impl AsRef<[T]>>,
) -> bool
where
    T: num_traits::Float + DivAssign + AddAssign + MulAssign + num_traits::FromPrimitive + num_traits::Signed,
{
    let scale = match params.scale.as_ref() {
        Some(scale) => scale.as_ref(),
        None => {
            let scale = mean(abs(a.as_ref()));
            let is_abs = !scale.is_finite() || scale < params.tolerance;
            if is_abs { &[T::one()] } else { &[scale] }
        }
    };
    let n = a.as_ref().len();
    let diff = sum(div(
        circle_zip(a, b, |a, b| a.sub(b).abs()),
        mul(scale, [T::from_usize(n).unwrap()]),
    ));
    diff < params.tolerance
}
#[derive(Debug, Clone, Copy)]
pub struct AllEqParams<T, SliceT> {
    pub tolerance: T,
    pub scale: Option<SliceT>,
}
pub fn all_eq_no_scale<T>() -> Option<&'static [T]> {
    None
}

fn circle_zip<A, B, C>(
    a: impl AsRef<[A]>,
    b: impl AsRef<[B]>,
    reduce_one: impl Fn(A, B) -> C,
) -> Vec<C>
where
    A: Clone,
    B: Clone,
{
    let a = a.as_ref();
    let b = b.as_ref();
    if a.is_empty() || b.is_empty() {
        panic!();
    }
    let a_divides_b = a.len().is_multiple_of(b.len());
    let b_divides_a = b.len().is_multiple_of(a.len());
    if !a_divides_b && !b_divides_a {
        panic!();
    }
    let mut a_i = 0;
    let mut b_i = 0;
    let mut out = vec![];
    let mut loop_once = false;
    loop {
        let a_0 = a_i == 0;
        let b_0 = b_i == 0;
        if a_0 && b_0 && loop_once {
            break;
        }
        loop_once = true;
        let c = reduce_one(a[a_i].clone(), b[b_i].clone());
        out.push(c);
        a_i = wrapping_incr(a_i, a.len());
        b_i = wrapping_incr(b_i, b.len());
    }
    out
}
fn wrapping_incr(curr: usize, end: usize) -> usize {
    let new = curr + 1;
    if new == end { 0 } else { new }
}

/// ref: <https://github.com/wch/r-source/blob/trunk/src/main/duplicate.c#L375>
pub(crate) fn extend_to_len<T>(vector: impl AsRef<[T]>, length: usize) -> Vec<T>
where
    T: Clone,
{
    let vector = vector.as_ref();
    let in_divides_out = length.is_multiple_of(vector.len());
    if !in_divides_out {
        panic!();
    }
    let mut out = Vec::with_capacity(length);
    while out.len() != length {
        out.extend(vector.iter().cloned());
    }
    out
}

#[rustfmt::skip]
pub fn subset<T>(vector: impl AsRef<[T]>, filter: impl Fn(T) -> bool) -> Vec<T> where T: Clone {
    vector.as_ref().iter().filter(|&x| filter(x.clone())).cloned().collect() }
#[rustfmt::skip]
pub fn which(vector: impl AsRef<[bool]>) -> Vec<usize> {
    vector.as_ref().iter().enumerate().filter_map(|(i, x)| if *x { Some(i) } else { None }).collect() }

pub fn table<Slice, T>(vectors: impl AsRef<[Slice]>) -> HashMap<Vec<T>, usize>
where
    Slice: AsRef<[T]>,
    T: Eq + std::hash::Hash + Clone,
{
    let mut out = HashMap::new();
    for i in 0.. {
        let mut key = vec![];
        for vector in vectors.as_ref() {
            let vector = vector.as_ref();
            let k = vector[i].clone();
            key.push(k);
        }
        let count = out.entry(key).or_insert(0);
        *count += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::vector::{SeqParams, seq};

    use super::*;

    #[test]
    #[should_panic]
    fn not_aligned() {
        let a: &[i32] = &[1, 4];
        let b: &[i32] = &[1, 2, 3];
        add(a, b);
    }

    #[test]
    fn basics() {
        let a: &[i32] = &[1, 4];
        let b: &[i32] = &[1, 2, 3, 4];
        assert_eq!(add(a, b), &[2, 6, 4, 8]);
        assert_eq!(sub(a, b), &[0, 2, -2, 0]);
        assert_eq!(mul(a, b), &[1, 8, 3, 16]);
        assert_eq!(div(a, b), &[1, 2, 0, 1]);
        assert_eq!(pow(a, cast::<_, u32>(b)), &[1, 16, 1, 256]);
        #[rustfmt::skip]
        assert_eq!(
            pow(
                seq(SeqParams::from(1..=10)),
                cast::<_, u32>(&[1, 2]),
            ),
            &[1, 4, 3, 16, 5, 36, 7, 64, 9, 100]
        );
        assert_eq!(min(a), 1);
    }

    #[test]
    fn test_sqrt() {
        #[rustfmt::skip]
        let x: &[f32] = &cast::<_, f32>(&seq(SeqParams::from(1..=6)));
        let sqrt = &sqrt(x);
        #[allow(clippy::approx_constant)]
        let y: &[f32] = &[1., 1.414214, 1.732051, 2., 2.236068, 2.44949];
        let d = &sub(sqrt, y);
        let d = &abs(d);
        let max = max(d);
        dbg!(max);
        assert!(max < 0.001);
    }

    #[test]
    fn test_mean() {
        #[rustfmt::skip]
        let x = &seq(SeqParams::from(1..=6));
        let x = &cast::<_, f32>(x);
        let mean = mean(x);
        assert_eq!(mean, 3.5);
    }

    #[test]
    fn not_finite() {
        let x: &[f32] = &[1., f32::NAN];
        assert!(min(x).is_nan());
        let x: &[f32] = &[1., -f32::INFINITY];
        assert_eq!(min(x), -f32::INFINITY);
        let x: &[f32] = &[1., f32::INFINITY];
        assert_eq!(max(x), f32::INFINITY);
        let x: &[f32] = &[1., f32::INFINITY, f32::INFINITY];
        assert_eq!(sort(x), &[1., f32::INFINITY, f32::INFINITY]);
    }

    #[test]
    fn nan() {
        let a = &[11., f64::NAN, 13.];
        assert_eq!(is_nan(a), &[false, true, false]);
        assert!(any(is_nan(a)));
        assert!(mean(a).is_nan());
        let b = &rm_nan(a);
        assert_eq!(mean(b), 12.);
    }

    #[test]
    fn bool() {
        let a = &neq([0], [0, 0, 1, 1]);
        let b = &neq([0], [0, 1, 0, 1]);
        assert_eq!(or(a, b), [false, true, true, true]);
        assert_eq!(xor(a, b), [false, true, true, false]);
        let x = &[1, 1, 2, 3, 5, 8, 13];
        assert_eq!(which(eq(modulo(x, [2]), [0])), [2, 5]);
    }

    #[test]
    fn test_tolerance() {
        let sqrt2 = &sqrt([2.]);
        let about2 = &mul(sqrt2, sqrt2);
        #[rustfmt::skip]
        assert!(all_eq(about2, [2.], &AllEqParams {
            tolerance: 10e-7, scale: all_eq_no_scale() }));
    }

    #[test]
    fn test_cumsum() {
        assert_eq!(cumsum([1, 2, 3, 4, 5]), [1, 3, 6, 10, 15]);
    }
}
