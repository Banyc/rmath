mod constructor;
mod ops;
use std::marker::PhantomData;

pub use constructor::*;
pub use ops::*;

/// ref: <https://github.com/wch/r-source/blob/67e3ab91b0489f56520142ce9352d68aa9a49ab0/src/main/array.c#L232>
#[derive(Debug, Clone)]
pub struct MatrixBuf<T> {
    dim: [usize; 2],
    data: Vec<T>,
}
impl<T> MatrixBuf<T> {
    pub fn dim(&self) -> [usize; 2] {
        self.dim
    }
    pub(crate) fn data(&self) -> &[T] {
        &self.data
    }
    pub(crate) fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
impl<T> AsRef<Self> for MatrixBuf<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub fn entry<T>(matrix: impl AsRef<MatrixBuf<T>>, index: [usize; 2]) -> T
where
    T: Copy,
{
    let matrix = matrix.as_ref();
    let dim = matrix.dim();
    matrix.data()[entry_index(&EntryIndexParams { index, dim })]
}
pub fn entry_mut<T>(matrix: &mut MatrixBuf<T>, index: [usize; 2]) -> &mut T {
    let dim = matrix.dim();
    &mut matrix.data_mut()[entry_index(&EntryIndexParams { index, dim })]
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EntryIndexParams {
    pub index: [usize; 2],
    pub dim: [usize; 2],
}
fn entry_index(params: &EntryIndexParams) -> usize {
    assert!(params.index[0] < params.dim[0]);
    assert!(params.index[1] < params.dim[1]);
    params.index[0] + params.index[1] * params.dim[0]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubmatrixParams {
    pub index: [usize; 2],
    pub dim: [usize; 2],
}
pub fn submatrix<T>(
    matrix: impl AsRef<MatrixBuf<T>>,
    index: [SubmatrixAxis<impl AsRef<[usize]>>; 2],
) -> MatrixBuf<T>
where
    T: Copy,
{
    let matrix = matrix.as_ref();
    let index_0 = || SubmatrixAxisIter::new(&index[0], 0..matrix.dim()[0]);
    let index_1 = || SubmatrixAxisIter::new(&index[1], 0..matrix.dim()[1]);
    let mut out = vec![];
    let dim = [index_0().count(), index_1().count()];
    for row_i in index_1() {
        for col_i in index_0() {
            out.push(entry(matrix, [col_i, row_i]));
        }
    }
    assert_eq!(dim[0] * dim[1], out.len());
    MatrixBuf { dim, data: out }
}
#[derive(Debug, Clone, Copy)]
pub enum SubmatrixAxis<Vec> {
    All,
    At(Vec),
}
enum SubmatrixAxisIter<'a> {
    All { range: std::ops::Range<usize> },
    At { i: usize, vec: &'a [usize] },
}
impl<'a> SubmatrixAxisIter<'a> {
    pub fn new(
        axis: &'a SubmatrixAxis<impl AsRef<[usize]>>,
        all_range: std::ops::Range<usize>,
    ) -> Self {
        match axis {
            SubmatrixAxis::All => Self::All { range: all_range },
            SubmatrixAxis::At(vec) => Self::At {
                i: 0,
                vec: vec.as_ref(),
            },
        }
    }
}
impl Iterator for SubmatrixAxisIter<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SubmatrixAxisIter::All { range } => range.next(),
            SubmatrixAxisIter::At { i, vec } => {
                let a = *vec.get(*i)?;
                *i += 1;
                Some(a)
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            SubmatrixAxisIter::All { range } => range.size_hint(),
            SubmatrixAxisIter::At { i, vec } => {
                let len = vec.len() - *i;
                (len, Some(len))
            }
        }
    }
}

#[derive(Debug)]
pub struct MatrixDisplay<M, T> {
    matrix: M,
    entry: PhantomData<T>,
}
impl<M, T> MatrixDisplay<M, T> {
    pub fn new(matrix: M) -> Self {
        Self {
            matrix,
            entry: PhantomData,
        }
    }
}
impl<M, T> std::fmt::Display for MatrixDisplay<M, T>
where
    M: AsRef<MatrixBuf<T>>,
    T: std::fmt::Display + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut col_len: Vec<usize> = vec![];
        let mut entries = vec![];
        for row_i in 0..self.matrix.as_ref().dim()[1] {
            for col_i in 0..self.matrix.as_ref().dim()[0] {
                let entry = entry(&self.matrix, [col_i, row_i]);
                let entry = entry.to_string();
                match col_len.get(col_i) {
                    Some(curr_len) => {
                        if *curr_len < entry.len() {
                            col_len[col_i] = entry.len();
                        }
                    }
                    None => col_len.push(entry.len()),
                }
                entries.push(entry);
            }
        }
        let mut col_i = 0;
        for entry in &entries {
            if 0 < col_i {
                write!(f, " ")?;
            }
            let col_len = col_len[col_i];
            write!(f, "{entry:>col_len$}")?;
            col_i += 1;
            if col_i == self.matrix.as_ref().dim()[0] {
                col_i = 0;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::{self, SeqParams, seq};

    use super::*;

    #[test]
    fn test_submatrix() {
        let a = vector::seq(&SeqParams {
            start: 1,
            end: 6,
            step: 1,
        });
        let mut a = matrix(&a, [3, 2], FillOrdering::RowByRow);
        *entry_mut(&mut a, [2, 0]) = 0;
        println!("{}", MatrixDisplay::new(&a));
        #[rustfmt::skip]
        let b = submatrix(
            &a,
            [
                SubmatrixAxis::At(seq(&SeqParams { start: 1, end: 2, step: 1 })),
                SubmatrixAxis::All,
            ],
        );
        println!("{}", MatrixDisplay::new(&b));
    }
}
