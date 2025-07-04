use crate::disjoint_type::{Float, Integer};

macro_rules! mass_impl {
    (
        impl $Trait: ident for { $( $Type: ty ),* }
            $body: tt
    ) => {
        $(
            impl $Trait for $Type
                $body
        )*
    };
}

pub trait IsNumType {
    fn is_float() -> bool;
    fn is_integer() -> bool;
}
mass_impl!(
    impl IsNumType for { u8, i8, u16, i16, u32, i32, u128, i128, usize, isize } {
        fn is_float() -> bool {
            false
        }
        fn is_integer() -> bool {
            true
        }
    }
);
mass_impl!(
    impl IsNumType for { f32, f64 } {
        fn is_float() -> bool {
            true
        }
         fn is_integer() -> bool {
            false
        }
    }
);

pub trait IsOrd: Sized {
    fn is_ord(&self) -> bool;
}
impl<F> IsOrd for Float<F>
where
    F: num_traits::Float,
{
    fn is_ord(&self) -> bool {
        !self.0.is_nan()
    }
}
impl<T> IsOrd for Integer<T>
where
    T: num_traits::PrimInt,
{
    fn is_ord(&self) -> bool {
        true
    }
}
mass_impl!(
    impl IsOrd for { u8, i8, u16, i16, u32, i32, u128, i128, usize, isize } {
        fn is_ord(&self) -> bool {
            true
        }
    }
);
mass_impl!(
    impl IsOrd for { f32, f64 } {
        fn is_ord(&self) -> bool {
            !self.is_nan()
        }
    }
);

#[allow(dead_code)]
pub trait IsFinite: Sized {
    fn is_finite(&self) -> bool;
}
impl<F> IsFinite for Float<F>
where
    F: num_traits::Float,
{
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
}
impl<T> IsFinite for Integer<T>
where
    T: num_traits::PrimInt,
{
    fn is_finite(&self) -> bool {
        true
    }
}
