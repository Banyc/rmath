use std::{iter::repeat_n, mem::MaybeUninit};

use crate::{
    matrix::{MatrixBuf, entry, entry_mut},
    vector,
};

/// ref: <https://github.com/wch/r-source/blob/67e3ab91b0489f56520142ce9352d68aa9a49ab0/src/main/array.c#L81>
pub fn matrix<T>(data: impl AsRef<[T]>, dim: [usize; 2], ordering: FillOrdering) -> MatrixBuf<T>
where
    T: Copy,
{
    let data = data.as_ref();
    if (usize::MAX as f64) < (dim[0] as f64) * (dim[1] as f64) {
        panic!();
    }
    if data.is_empty() {
        panic!();
    }
    let out_len = dim[0] * dim[1];
    let in_divides_out = out_len.is_multiple_of(data.len());
    if !in_divides_out {
        panic!();
    }
    // ref: <https://github.com/wch/r-source/blob/trunk/src/main/duplicate.c#L446>
    match ordering {
        FillOrdering::RowByRow => {
            let matrix = vector::extend_to_len(data, out_len);
            MatrixBuf { dim, data: matrix }
        }
        FillOrdering::ColByCol => {
            // ref: <https://github.com/wch/r-source/blob/trunk/src/main/duplicate.h#L61>
            let mut matrix = Vec::with_capacity(out_len);
            matrix.extend(repeat_n(MaybeUninit::uninit(), out_len));
            let mut matrix = MatrixBuf { dim, data: matrix };
            let mut in_i = 0;
            for col_i in 0..dim[0] {
                for row_i in 0..dim[1] {
                    *entry_mut(&mut matrix, [col_i, row_i]) = MaybeUninit::new(data[in_i]);
                    in_i = (in_i + 1) % data.len();
                }
            }
            unsafe { std::mem::transmute::<MatrixBuf<MaybeUninit<T>>, MatrixBuf<T>>(matrix) }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillOrdering {
    RowByRow,
    ColByCol,
}

pub fn diag<T>(data: impl AsRef<[T]>) -> MatrixBuf<T>
where
    T: num_traits::Zero + Copy,
{
    let data = data.as_ref();
    if (usize::MAX as f64) < (data.len() as f64).powi(2) {
        panic!();
    }
    if data.is_empty() {
        panic!();
    }
    let dim = [data.len(), data.len()];
    let out_len = data.len().pow(2);
    let mut matrix = Vec::with_capacity(out_len);
    matrix.extend(std::iter::repeat_n(T::zero(), out_len));
    let mut matrix = MatrixBuf { dim, data: matrix };
    for (i, item) in data.iter().enumerate() {
        *entry_mut(&mut matrix, [i, i]) = *item;
    }
    matrix
}

/// stack vertically
pub fn rbind<M, T>(matrices: impl AsRef<[M]>) -> MatrixBuf<T>
where
    T: Copy,
    M: AsRef<MatrixBuf<T>>,
{
    let mut dim: Option<[usize; 2]> = None;
    let matrices = matrices.as_ref();
    let mut vectors = vec![];
    for matrix in matrices {
        let matrix = matrix.as_ref();
        match &mut dim {
            Some(curr_dim) => {
                assert_eq!(curr_dim[0], matrix.dim()[0]);
                curr_dim[1] += matrix.dim()[1];
            }
            None => dim = Some(matrix.dim()),
        }
        vectors.push(matrix.data());
    }
    let matrix = vector::c(&vectors);
    MatrixBuf {
        dim: dim.unwrap(),
        data: matrix,
    }
}

/// stack horizontally
pub fn cbind<M, T>(matrices: impl AsRef<[M]>) -> MatrixBuf<T>
where
    T: Copy,
    M: AsRef<MatrixBuf<T>>,
{
    let mut dim: Option<[usize; 2]> = None;
    let matrices = matrices.as_ref();
    for matrix in matrices {
        let matrix = matrix.as_ref();
        match &mut dim {
            Some(curr_dim) => {
                assert_eq!(curr_dim[1], matrix.dim()[1]);
                curr_dim[0] += matrix.dim()[0];
            }
            None => dim = Some(matrix.dim()),
        }
    }
    let dim = dim.unwrap();
    let mut out = vec![];
    for row_i in 0..dim[1] {
        for matrix in matrices {
            let matrix = matrix.as_ref();
            for inner_col_i in 0..matrix.dim()[0] {
                out.push(entry(matrix, [inner_col_i, row_i]));
            }
        }
    }
    MatrixBuf { dim, data: out }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let a = &[0, 1];
        let b = matrix(a, [2, 3], FillOrdering::RowByRow);
        assert_eq!(b.data(), [0, 1, 0, 1, 0, 1]);
        let c = matrix(a, [3, 2], FillOrdering::ColByCol);
        assert_eq!(c.data(), [0, 0, 0, 1, 1, 1]);
        let d = diag(a);
        assert_eq!(d.data(), [0, 0, 0, 1]);
        let e = rbind([&b, &d]);
        assert_eq!(e.dim(), [2, 5]);
        assert_eq!(e.data(), [0, 1, 0, 1, 0, 1, 0, 0, 0, 1]);
        let f = cbind([&c, &d]);
        assert_eq!(f.dim(), [5, 2]);
        assert_eq!(f.data(), [0, 0, 0, 0, 0, 1, 1, 1, 0, 1]);
    }
}
