mod constructor;
pub use constructor::*;
mod ops;
pub use ops::*;

pub fn set<T>(
    dst: &mut Vec<T>,
    index: impl std::ops::RangeBounds<usize>,
    src: impl AsRef<[T]>,
    fill: impl Fn() -> T,
) where
    T: Clone,
{
    let start = match index.start_bound() {
        std::ops::Bound::Included(x) => *x,
        std::ops::Bound::Excluded(x) => *x + 1,
        std::ops::Bound::Unbounded => 0,
    };
    let end = match index.end_bound() {
        std::ops::Bound::Included(x) => *x + 1,
        std::ops::Bound::Excluded(x) => *x,
        std::ops::Bound::Unbounded => dst.len(),
    };
    let src = src.as_ref();
    let dst_len = end.checked_sub(start).unwrap();
    assert!(dst_len.is_multiple_of(src.len()));
    if start < dst.len() {
        let new_end = dst.len().min(end);
        let total_copy_len = new_end - start;
        let mut pos = 0;
        while pos < total_copy_len {
            let copy_len = (total_copy_len - pos).min(src.len());
            dst[start + pos..start + pos + copy_len].clone_from_slice(&src[..copy_len]);
            pos += copy_len;
        }
        return;
    }
    let fill_count = end - dst.len();
    dst.extend(std::iter::repeat_n(fill(), fill_count));
    dst[start..end].clone_from_slice(src);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let mut a = seq(SeqParams::from(1..=2));
        #[rustfmt::skip]
        set(&mut a, 0..=0, [0], || 0);
        assert_eq!(a, [0, 2]);
        #[rustfmt::skip]
        set(&mut a, 3..=3, [3], || 0);
        assert_eq!(a, [0, 2, 0, 3]);
    }
}
