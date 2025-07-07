mod constructor;
pub use constructor::*;
mod ops;
pub use ops::*;

#[derive(Debug, Clone, Copy)]
pub struct SetParams<T> {
    pub index: usize,
    pub value: T,
    pub filler: T,
}
pub fn set<T>(vector: &mut Vec<T>, params: &SetParams<T>)
where
    T: Copy,
{
    if params.index < vector.len() {
        vector[params.index] = params.value;
        return;
    }
    let fill_count = params.index - vector.len();
    vector.extend(std::iter::repeat_n(params.filler, fill_count));
    vector.push(params.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let mut a = seq(&SeqParams {
            start: 1,
            end: 2,
            step: 1,
        });
        #[rustfmt::skip]
        set(&mut a, &SetParams { index: 0, value: 0, filler: 0 });
        assert_eq!(a, [0, 2]);
        #[rustfmt::skip]
        set(&mut a, &SetParams { index: 3, value: 3, filler: 0 });
        assert_eq!(a, [0, 2, 0, 3]);
    }
}
