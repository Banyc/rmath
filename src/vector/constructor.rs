use std::ops::AddAssign;

#[derive(Debug, Clone, Copy)]
pub struct SeqParams<T> {
    pub start: T,
    pub end: T,
    pub step: T,
}
pub fn seq<T>(params: &SeqParams<T>) -> Vec<T>
where
    T: Copy + PartialOrd + AddAssign,
{
    let mut vec = vec![];
    let mut curr = params.start;
    while curr <= params.end {
        vec.push(curr);
        curr += params.step;
    }
    vec
}

#[derive(Debug, Clone, Copy)]
pub struct RepParams<T> {
    pub value: T,
    pub times: usize,
}
pub fn rep<T>(params: &RepParams<T>) -> Vec<T>
where
    T: Copy,
{
    let mut vec = vec![];
    for _ in 0..params.times {
        vec.push(params.value);
    }
    vec
}

pub fn c<Slice2, Slice1, T>(vectors: Slice2) -> Vec<T>
where
    Slice2: AsRef<[Slice1]>,
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
        let x = seq(&SeqParams { start: 1, end: 20, step: 2 });
        let y = rep(&RepParams { value: 3, times: 4 });
        let z = c([&y, &x]);
        #[rustfmt::skip]
        assert_eq!(z, [3,3,3,3,1,3,5,7,9,11,13,15,17,19]);

        let x = seq(&SeqParams {
            start: 0.1,
            end: 0.9,
            step: 0.1,
        });
        assert_eq!(x.len(), 9);
    }
}
