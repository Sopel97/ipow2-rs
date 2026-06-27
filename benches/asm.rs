use ipow2::{
    CeilToMultiple, CheckedCeilToMultiple, CheckedDiv, CheckedDivCeil, CheckedDivFloor,
    CheckedDivRound, CheckedFloorToMultiple, CheckedModFloor, CheckedMul, CheckedRoundToMultiple,
    DivCeil, DivFloor, DivRound, FloorToMultiple, IsMultipleOf, ModFloor, Pow2, RoundToMultiple,
    UnboundedCeilToMultiple, UnboundedDiv, UnboundedDivCeil, UnboundedDivFloor, UnboundedDivRound,
    UnboundedFloorToMultiple, UnboundedIsMultipleOf, UnboundedPow2, UnboundedRoundToMultiple,
};
use paste::paste;
use std::ops::{Div, Mul};

macro_rules! impl_binop {
    ($trait:ident, $func:ident, $rhs:ty, $suffix:ident, $($t:ty),*) => {
        $(
            paste! {
                #[unsafe(no_mangle)]
                pub fn [<$func:lower _ $t:lower _ $suffix>](lhs: $t, rhs: $rhs) -> <$t as $trait<$rhs>>::Output {
                    <$t as $trait<$rhs>>::$func(lhs, rhs)
                }
            }
        )*
    };
}

macro_rules! impl_binops {
    ($rhs:ty, $suffix:ident, $([$trait:ident, $func:ident]),*) => {
        $(
            impl_binop!($trait, $func, $rhs, $suffix, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
        )*
    };
}

impl_binops!(
    Pow2<u8>,
    pow2,
    [Div, div],
    [Mul, mul],
    [CheckedMul, checked_mul],
    [CheckedCeilToMultiple, checked_ceil_to_multiple],
    [CeilToMultiple, ceil_to_multiple],
    [FloorToMultiple, floor_to_multiple],
    [RoundToMultiple, round_to_multiple],
    [IsMultipleOf, is_multiple_of],
    [DivCeil, div_ceil],
    [ModFloor, mod_floor],
    [DivFloor, div_floor],
    [DivRound, div_round]
);

impl_binops!(
    UnboundedPow2,
    unb_pow2,
    [Div, div],
    [CheckedDiv, checked_div],
    [UnboundedDiv, unbounded_div],
    [Mul, mul],
    [CheckedMul, checked_mul],
    [CeilToMultiple, ceil_to_multiple],
    [CheckedCeilToMultiple, checked_ceil_to_multiple],
    [UnboundedCeilToMultiple, unbounded_ceil_to_multiple],
    [FloorToMultiple, floor_to_multiple],
    [CheckedFloorToMultiple, checked_floor_to_multiple],
    [UnboundedFloorToMultiple, unbounded_floor_to_multiple],
    [RoundToMultiple, round_to_multiple],
    [CheckedRoundToMultiple, checked_round_to_multiple],
    [UnboundedRoundToMultiple, unbounded_round_to_multiple],
    [IsMultipleOf, is_multiple_of],
    [UnboundedIsMultipleOf, unbounded_is_multiple_of],
    [DivCeil, div_ceil],
    [CheckedDivCeil, checked_div_ceil],
    [UnboundedDivCeil, unbounded_div_ceil],
    [ModFloor, mod_floor],
    [CheckedModFloor, checked_mod_floor],
    [DivFloor, div_floor],
    [CheckedDivFloor, checked_div_floor],
    [UnboundedDivFloor, unbounded_div_floor],
    [DivRound, div_round],
    [CheckedDivRound, checked_div_round],
    [UnboundedDivRound, unbounded_div_round]
);
