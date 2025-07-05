use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use num_traits::{One, Pow, Zero};

use crate::property::{IsNumType, IsOrd};

pub fn cast<A, B>(a: impl AsRef<[A]>) -> Vec<B>
where
    A: Copy,
    B: TryFrom<A>,
{
    a.as_ref()
        .iter()
        .map(|a| (*a).try_into().unwrap_or_else(|_| panic!()))
        .collect()
}

pub fn sum<T>(vector: impl AsRef<[T]>) -> T
where
    T: Copy + AddAssign + Zero,
{
    vector.as_ref().iter().fold(T::zero(), |mut cum, a| {
        cum += *a;
        cum
    })
}
pub fn prod<T>(vector: impl AsRef<[T]>) -> T
where
    T: Copy + MulAssign + One,
{
    vector.as_ref().iter().fold(T::one(), |mut cum, a| {
        cum *= *a;
        cum
    })
}
pub fn min<T>(vector: impl AsRef<[T]>) -> T
where
    T: PartialOrd + IsOrd + Copy,
{
    let mut curr_min = None;
    for item in vector.as_ref() {
        let true = item.is_ord() else {
            // continue;
            return *item;
        };
        let Some(prev_curr_min) = curr_min else {
            curr_min = Some(item);
            continue;
        };
        if item < prev_curr_min {
            curr_min = Some(item);
        }
    }
    *curr_min.unwrap()
}
pub fn max<T>(vector: impl AsRef<[T]>) -> T
where
    T: PartialOrd + IsOrd + Copy,
{
    let mut curr_max = None;
    for item in vector.as_ref() {
        let true = item.is_ord() else {
            // continue;
            return *item;
        };
        let Some(prev_curr_max) = curr_max else {
            curr_max = Some(item);
            continue;
        };
        if prev_curr_max < item {
            curr_max = Some(item);
        }
    }
    *curr_max.unwrap()
}
pub fn sqrt<T>(vector: impl AsRef<[T]>) -> Vec<T>
where
    T: num_traits::Float,
{
    let mut out = vec![];
    for item in vector.as_ref() {
        out.push(item.sqrt())
    }
    out
}
pub fn abs<T>(vector: impl AsRef<[T]>) -> Vec<T>
where
    T: num_traits::Signed,
{
    let mut out = vec![];
    for item in vector.as_ref() {
        out.push(item.abs());
    }
    out
}
pub fn sort<T>(vector: impl AsRef<[T]>) -> Vec<T>
where
    T: IsOrd + IsNumType + Copy + PartialOrd,
{
    let vector = vector.as_ref();
    let mut out = vec![];
    if T::is_float() {
        for item in vector {
            if item.is_ord() {
                out.push(*item);
            }
        }
    } else {
        out = vector.to_vec();
    }
    out.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    out
}

pub fn add<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T>
where
    T: Copy + AddAssign,
{
    circle_zip(a, b, |mut a, b| {
        a += b;
        a
    })
}
pub fn sub<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T>
where
    T: Copy + SubAssign,
{
    circle_zip(a, b, |mut a, b| {
        a -= b;
        a
    })
}
pub fn mul<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T>
where
    T: Copy + MulAssign,
{
    circle_zip(a, b, |mut a, b| {
        a *= b;
        a
    })
}
pub fn div<T>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T>
where
    T: Copy + DivAssign,
{
    circle_zip(a, b, |mut a, b| {
        a /= b;
        a
    })
}
pub fn pow<A, B>(a: impl AsRef<[A]>, b: impl AsRef<[B]>) -> Vec<A>
where
    A: Copy + Pow<B, Output = A>,
    B: Copy,
{
    circle_zip(a, b, |a, b| a.pow(b))
}

fn circle_zip<A, B, C>(
    a: impl AsRef<[A]>,
    b: impl AsRef<[B]>,
    reduce_one: impl Fn(A, B) -> C,
) -> Vec<C>
where
    A: Copy,
    B: Copy,
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
        let c = reduce_one(a[a_i], b[b_i]);
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

#[cfg(test)]
mod tests {
    use crate::vector::{SeqParams, seq};

    use super::*;

    #[test]
    fn basics() {
        let a: &[i32] = &[1, 4];
        let b: &[i32] = &[1, 2, 3, 4];
        assert_eq!(add(a, b), &[2, 6, 4, 8]);
        assert_eq!(sub(a, b), &[0, 2, -2, 0]);
        assert_eq!(mul(a, b), &[1, 8, 3, 16]);
        assert_eq!(div(a, b), &[1, 2, 0, 1]);
        assert_eq!(pow(a, cast::<_, u32>(b)), &[1, 16, 1, 256]);
        assert_eq!(
            pow(
                seq(&SeqParams {
                    start: 1,
                    end: 10,
                    step: 1
                }),
                cast::<_, u32>(&[1, 2]),
            ),
            &[1, 4, 3, 16, 5, 36, 7, 64, 9, 100]
        );
        assert_eq!(min(a), 1);
    }

    #[test]
    fn test_sqrt() {
        let x: &[f32] = &seq(&SeqParams {
            start: 1.,
            end: 6.,
            step: 1.,
        });
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
    fn not_finite() {
        let x: &[f32] = &[1., f32::NAN];
        assert!(min(x).is_nan());
        let x: &[f32] = &[1., -f32::INFINITY];
        assert_eq!(min(x), -f32::INFINITY);
        let x: &[f32] = &[1., f32::INFINITY];
        assert_eq!(max(x), f32::INFINITY);
    }
}
