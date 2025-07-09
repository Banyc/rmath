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

pub trait IsOrd: Sized {
    fn always_ord() -> bool;
    fn is_ord(&self) -> bool;
}
mass_impl!(
    impl IsOrd for { u8, i8, u16, i16, u32, i32, u128, i128, usize, isize } {
        fn always_ord() -> bool {
            true
        }
        fn is_ord(&self) -> bool {
            true
        }
    }
);
mass_impl!(
    impl IsOrd for { f32, f64 } {
        fn always_ord() -> bool {
            false
        }
        fn is_ord(&self) -> bool {
            !self.is_nan()
        }
    }
);

#[rustfmt::skip]
pub trait CalcFactorial {
    fn factorial(&self) -> Self where Self: Sized;
}
mass_impl!(
    impl CalcFactorial for { u8, i8, u16, i16, u32, i32, u128, i128, usize, isize } {
        fn factorial(&self) -> Self {
            if *self == 0 {
                return 1;
            }
            (1..=*self).product()
        }
    }
);
mass_impl!(
    impl CalcFactorial for { f32, f64 } {
        #[allow(clippy::useless_conversion)]
        fn factorial(&self) -> Self {
            statrs::function::gamma::gamma((self + 1.).into()) as _
        }
    }
);

#[rustfmt::skip]
pub trait CalcChoose {
    fn choose(&self, r: Self) -> Self where Self: Sized;
}
mass_impl!(
    impl CalcChoose for { u8, i8, u16, i16, u32, i32, u128, i128, usize, isize } {
        fn choose(&self, r: Self) -> Self {
            assert!(r <= *self);
            let a = *self - r + 1;
            let a: Self = (a..=*self).product();
            a / r.factorial()
        }
    }
);
mass_impl!(
    impl CalcChoose for { f32, f64 } {
        fn choose(&self, r: Self) -> Self {
            self.factorial() / (r.factorial() * (*self - r).factorial())
        }
    }
);

#[cfg(test)]
mod tests {
    use crate::vector;

    use super::*;

    #[rustfmt::skip]
    #[test]
    fn test_choose() {
        assert_eq!(4.choose(2), 6);
        assert!(vector::all_eq([(4.0_f64).choose(2.)], [6.], &vector::AllEqParams { tolerance: 10e-7, scale: vector::all_eq_no_scale() }));
        assert_eq!(6.choose(4), 15);
        assert!(vector::all_eq([(6.0_f64).choose(4.)], [15.], &vector::AllEqParams { tolerance: 10e-7, scale: vector::all_eq_no_scale() }));
    }
}
