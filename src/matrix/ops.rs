use std::mem::MaybeUninit;

use crate::{
    matrix::{FillOrdering, MatrixBuf, entry, entry_mut, matrix},
    vector,
};

pub fn cast<A, B>(input: impl AsRef<MatrixBuf<A>>) -> MatrixBuf<B>
where
    A: Copy + num_traits::AsPrimitive<B>,
    B: Copy + 'static,
{
    let input = input.as_ref();
    let data = vector::cast(input.data());
    MatrixBuf {
        dim: input.dim(),
        data,
    }
}

pub use single_vector_in_single_vector_out::*;
#[rustfmt::skip]
mod single_vector_in_single_vector_out {
    use super::*;
    pub fn neg<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: std::ops::Neg<Output = T> + Copy {
        elem_op1(matrix, |x| vector::neg(x)) }
    pub fn not(matrix: impl AsRef<MatrixBuf<bool>>) -> MatrixBuf<bool> {
        elem_op1(matrix, |x| vector::not(x)) }
    pub fn exp<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: num_traits::Float {
        elem_op1(matrix, |x| vector::exp(x)) }
    pub fn sqrt<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: num_traits::Float {
        elem_op1(matrix, |x| vector::sqrt(x)) }
    pub fn abs<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: num_traits::Signed {
        elem_op1(matrix, |x| vector::abs(x)) }
    pub fn sin<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: num_traits::Float {
        elem_op1(matrix, |x| vector::sin(x)) }
    pub fn cos<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: num_traits::Float {
        elem_op1(matrix, |x| vector::cos(x)) }
    pub fn tan<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: num_traits::Float {
        elem_op1(matrix, |x| vector::tan(x)) }
    pub fn is_nan<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<bool> where T: num_traits::Float {
        elem_op1(matrix, |x| vector::is_nan(x)) }
    pub fn rm_nan<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: num_traits::Float {
        elem_op1(matrix, |x| vector::rm_nan(x)) }
    pub fn floor<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: num_traits::Float {
        elem_op1(matrix, |x| vector::floor(x)) }
    pub fn ceil<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: num_traits::Float {
        elem_op1(matrix, |x| vector::ceil(x)) }
    pub fn round<T>(matrix: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: num_traits::Float {
        elem_op1(matrix, |x| vector::round(x)) }
}
fn elem_op1<A, B>(
    input: impl AsRef<MatrixBuf<A>>,
    vec_op: impl Fn(&[A]) -> Vec<B>,
) -> MatrixBuf<B> {
    let input = input.as_ref();
    let data = vec_op(input.data());
    MatrixBuf {
        dim: input.dim(),
        data,
    }
}

pub use two_vectors_in_single_vector_out::*;
#[rustfmt::skip]
mod two_vectors_in_single_vector_out {
    use super::*;
    pub fn add<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: Copy + std::ops::AddAssign {
        elem_op2(a, b, |a, b| vector::add(a, b)) }
    pub fn sub<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: Copy + std::ops::SubAssign {
        elem_op2(a, b, |a, b| vector::sub(a, b)) }
    pub fn mul<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: Copy + std::ops::MulAssign {
        elem_op2(a, b, |a, b| vector::mul(a, b)) }
    pub fn div<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: Copy + std::ops::DivAssign {
        elem_op2(a, b, |a, b| vector::div(a, b)) }
    pub fn modulo<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T> where T: Copy + num_traits::PrimInt {
        elem_op2(a, b, |a, b| vector::modulo(a, b)) }
    pub fn pow<A, B>(a: impl AsRef<MatrixBuf<A>>, b: impl AsRef<MatrixBuf<B>>) -> MatrixBuf<A> where A: Copy + num_traits::Pow<B, Output = A>, B: Copy {
        elem_op2(a, b, |a, b| vector::pow(a, b)) }
    pub fn lt<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<bool> where T: PartialOrd + Copy {
        elem_op2(a, b, |a, b| vector::lt(a, b)) }
    pub fn gt<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<bool> where T: PartialOrd + Copy {
        elem_op2(a, b, |a, b| vector::gt(a, b)) }
    pub fn eq<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<bool> where T: PartialEq + Copy {
        elem_op2(a, b, |a, b| vector::eq(a, b)) }
    pub fn neq<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<bool> where T: PartialEq + Copy {
        elem_op2(a, b, |a, b| vector::neq(a, b)) }
    pub fn or(a: impl AsRef<MatrixBuf<bool>>, b: impl AsRef<MatrixBuf<bool>>) -> MatrixBuf<bool> {
        elem_op2(a, b, |a, b| vector::or(a, b)) }
    pub fn and(a: impl AsRef<MatrixBuf<bool>>, b: impl AsRef<MatrixBuf<bool>>) -> MatrixBuf<bool> {
        elem_op2(a, b, |a, b| vector::and(a, b)) }
    pub fn xor(a: impl AsRef<MatrixBuf<bool>>, b: impl AsRef<MatrixBuf<bool>>) -> MatrixBuf<bool> {
        elem_op2(a, b, |a, b| vector::xor(a, b)) }
}
fn elem_op2<A, B, C>(
    a: impl AsRef<MatrixBuf<A>>,
    b: impl AsRef<MatrixBuf<B>>,
    vec_op: impl Fn(&[A], &[B]) -> Vec<C>,
) -> MatrixBuf<C> {
    let a = a.as_ref();
    let b = b.as_ref();
    for (i, (a, b)) in a.dim().into_iter().zip(b.dim()).enumerate() {
        let a_divides_b = b.is_multiple_of(a);
        let b_divides_a = a.is_multiple_of(b);
        if !a_divides_b && !b_divides_a {
            panic!("{i}");
        }
    }
    let data = vec_op(a.data(), b.data());
    let dim = std::array::from_fn(|i| a.dim()[i].max(b.dim()[i]));
    MatrixBuf { dim, data }
}

#[rustfmt::skip]
pub fn map<A, B>(matrix: impl AsRef<MatrixBuf<A>>, fmap: impl Fn(A) -> B) -> MatrixBuf<B> where A: Copy {
    let matrix = matrix.as_ref();
    MatrixBuf { dim: matrix.dim(), data: vector::map(matrix.data(), fmap) } }

#[rustfmt::skip]
pub fn all_eq<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>, params: &vector::AllEqParams<T, impl AsRef<[T]>>) -> bool
where
    T: num_traits::Float + std::ops::DivAssign + std::ops::AddAssign + std::ops::MulAssign + num_traits::FromPrimitive + num_traits::Signed,
{
    let a = a.as_ref();
    let b = b.as_ref();
    if a.dim() != b.dim() {
        return false;
    }
    vector::all_eq(a.data(), b.data(), params)
}

pub fn t<T>(input: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T>
where
    T: Copy,
{
    let input = input.as_ref();
    let out_dim = [input.dim()[1], input.dim()[0]];
    let mut out = matrix([MaybeUninit::uninit()], out_dim, FillOrdering::RowByRow);
    for row_i in 0..input.dim()[1] {
        for col_i in 0..input.dim()[0] {
            *entry_mut(&mut out, [row_i, col_i]) = MaybeUninit::new(entry(input, [col_i, row_i]));
        }
    }
    unsafe { std::mem::transmute::<MatrixBuf<MaybeUninit<T>>, MatrixBuf<T>>(out) }
}

/// ref: <https://github.com/wch/r-source/blob/426f1a30b40ed63a2f915bce1a6e69d1bef167da/src/library/base/R/det.R#L25>
pub fn det<T>(input: impl AsRef<MatrixBuf<T>>) -> T
where
    T: nalgebra::Scalar + nalgebra::ComplexField + Copy,
{
    let input = input.as_ref();
    assert_eq!(input.dim()[0], input.dim()[1]);
    matrix_buf_to_malgebra(input).determinant()
}

pub fn solve1<T>(input: impl AsRef<MatrixBuf<T>>) -> Option<MatrixBuf<T>>
where
    T: nalgebra::ComplexField + Copy,
{
    let input = input.as_ref();
    assert_eq!(input.dim()[0], input.dim()[1]);
    let m = matrix_buf_to_malgebra(input).try_inverse()?;
    Some(malgebra_to_matrix_buf(m))
}
pub fn solve2<T>(a: impl AsRef<MatrixBuf<T>>, b: impl AsRef<MatrixBuf<T>>) -> MatrixBuf<T>
where
    T: nalgebra::ComplexField + Copy,
{
    let a = a.as_ref();
    let b = b.as_ref();
    assert_eq!(a.dim()[0], b.dim()[1]);
    let a_ = matrix_buf_to_malgebra(a);
    let b_ = matrix_buf_to_malgebra(b);
    let m = a_ * b_;
    let m = malgebra_to_matrix_buf(m);
    assert_eq!(m.dim()[0], b.dim()[0]);
    assert_eq!(m.dim()[1], a.dim()[1]);
    m
}

fn matrix_buf_to_malgebra<T>(m: impl AsRef<MatrixBuf<T>>) -> nalgebra::DMatrix<T>
where
    T: nalgebra::Scalar + Copy,
{
    let m = m.as_ref();
    nalgebra::DMatrix::from_fn(m.dim()[1], m.dim()[0], |r, c| entry(m, [c, r]))
}
fn malgebra_to_matrix_buf<T, R, C, S>(m: nalgebra::Matrix<T, R, C, S>) -> MatrixBuf<T>
where
    T: Copy,
    R: nalgebra::Dim,
    C: nalgebra::Dim,
    S: nalgebra::RawStorage<T, R, C>,
{
    let dim = [m.shape().1, m.shape().0];
    let mut data = vec![];
    for row_i in 0..dim[1] {
        for col_i in 0..dim[0] {
            data.push(*m.get((row_i, col_i)).unwrap());
        }
    }
    MatrixBuf { dim, data }
}

#[cfg(test)]
mod tests {
    use crate::vector::{SeqParams, seq};

    use super::*;

    #[rustfmt::skip]
    #[test]
    fn test_t() {
        let a = matrix(seq(SeqParams::from(1..=6)), [3, 2], FillOrdering::RowByRow);
        let b = t(&a);
        assert_eq!(b.dim(), [2, 3]);
        assert_eq!(b.data(), [1, 4, 2, 5, 3, 6]);
    }

    #[test]
    fn test_det() {
        let a = matrix([1, -2, 3, 2, 0, 3, 1, 5, 4], [3, 3], FillOrdering::RowByRow);
        let a = cast::<_, f64>(a);
        assert_eq!(det(a), 25.);
    }

    #[test]
    fn test_inv() {
        let a = matrix([3, 2, 5, 3], [2, 2], FillOrdering::RowByRow);
        let a = cast::<_, f64>(a);
        assert_eq!(solve1(a).unwrap().data(), [-3., 2., 5., -3.]);
    }

    #[test]
    fn test_mul() {
        let a = matrix([1, 2, 3, 4, 5, 6], [3, 2], FillOrdering::RowByRow);
        let b = matrix([7, 8, 9, 10, 11, 12], [2, 3], FillOrdering::RowByRow);
        let a = cast::<_, f64>(a);
        let b = cast::<_, f64>(b);
        let m = solve2(a, b);
        assert_eq!(m.data(), [58., 64., 139., 154.]);
        assert_eq!(m.dim(), [2, 2]);
    }
}
