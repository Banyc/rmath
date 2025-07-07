use std::ops::AddAssign;

#[derive(Debug, Clone, Copy)]
pub struct SeqParams<T> {
    pub start: T,
    pub end: T,
    pub step: T,
}
pub fn seq<T>(params: impl AsRef<SeqParams<T>>) -> Vec<T>
where
    T: Copy + PartialOrd + AddAssign,
{
    let params = params.as_ref();
    let mut vec = vec![];
    let mut curr = params.start;
    while curr <= params.end {
        vec.push(curr);
        curr += params.step;
    }
    vec
}
#[rustfmt::skip]
impl<T> AsRef<SeqParams<T>> for SeqParams<T> { fn as_ref(&self) -> &SeqParams<T> { self } }
#[rustfmt::skip]
impl<T> From<std::ops::Range<T>> for SeqParams<T> where T: num_traits::PrimInt {
    fn from(value: std::ops::Range<T>) -> Self {
        Self {
            start: value.start,
            end: value.end.sub(T::one()),
            step: T::one(),
        } } }
#[rustfmt::skip]
impl<T> From<std::ops::RangeInclusive<T>> for SeqParams<T> where T: num_traits::PrimInt {
    fn from(value: std::ops::RangeInclusive<T>) -> Self {
        Self {
            start: *value.start(),
            end: *value.end(),
            step: T::one(),
        } } }

#[derive(Debug, Clone, Copy)]
pub struct RepParams<T> {
    pub value: T,
    pub times: usize,
}
pub fn rep<T>(params: impl AsRef<RepParams<T>>) -> Vec<T>
where
    T: Copy,
{
    let params = params.as_ref();
    vec![params.value; params.times]
}
#[rustfmt::skip]
impl<T> AsRef<RepParams<T>> for RepParams<T> { fn as_ref(&self) -> &RepParams<T> { self } }

pub fn c<Slice1, T>(vectors: impl AsRef<[Slice1]>) -> Vec<T>
where
    Slice1: AsRef<[T]>,
    T: Copy,
{
    let mut vec = vec![];
    for vector in vectors.as_ref() {
        vec.extend(vector.as_ref());
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        #[rustfmt::skip]
        let x = seq(SeqParams { start: 1, end: 20, step: 2 });
        let y = rep(RepParams { value: 3, times: 4 });
        let z = c([&y, &x]);
        #[rustfmt::skip]
        assert_eq!(z, [3,3,3,3,1,3,5,7,9,11,13,15,17,19]);

        let x = seq(SeqParams {
            start: 0.1,
            end: 0.9,
            step: 0.1,
        });
        assert_eq!(x.len(), 9);
    }
}
