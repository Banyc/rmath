pub trait AsInner: Sized + From<Self::Inner> {
    type Inner: From<Self>;
}

macro_rules! wrap {
    ($WrapType: ident { $($InnerType: ident),* }) => {
        $(
            impl From<$InnerType> for $WrapType<$InnerType> {
                fn from(value: $InnerType) -> $WrapType<$InnerType> {
                    $WrapType(value)
                }
            }
            impl From<$WrapType<$InnerType>> for $InnerType {
                fn from(value: $WrapType<$InnerType>) -> $InnerType {
                    value.0
                }
            }
            impl AsInner for $WrapType<$InnerType> {
                type Inner = $InnerType;
            }
        )*
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Float<T: num_traits::Float>(pub T);
#[derive(Debug, Clone, Copy)]
pub struct Integer<T: num_traits::PrimInt>(pub T);
wrap!(Float { f64, f32 });
#[rustfmt::skip]
wrap!(Integer { u8, i8, u16, i16, u32, i32, u128, i128, usize, isize });
