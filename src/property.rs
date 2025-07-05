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
