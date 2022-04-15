// False positive in derived code
#![allow(unused_qualifications)]

// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

#[cfg(feature = "parity_codec")]
use parity_scale_codec::{Decode, Encode};
#[cfg(any(test, feature = "proptest"))]
use proptest_derive::Arbitrary;
use std::{cmp::Ordering, u64};
use crate::MulFullInline;
use crate::multiplicative::mul_full_inline_const;

#[derive(PartialEq, Eq, Clone, Default, Hash, Copy)]
#[cfg_attr(feature = "parity_codec", derive(Encode, Decode))]
// TODO: Generate a quasi-random sequence.
// See http://extremelearning.com.au/unreasonable-effectiveness-of-quasirandom-sequences/
#[cfg_attr(any(test, feature = "proptest"), derive(Arbitrary))]
#[cfg_attr(any(test, feature = "proptest"), proptest(no_params))]
pub struct U256([u64; 4]);

// TODO: impl core::iter::Step so we have ranges

impl U256 {
    pub const MAX: Self = Self::from_limbs([
        u64::max_value(),
        u64::max_value(),
        u64::max_value(),
        u64::max_value(),
    ]);
    pub const ONE: Self = Self::from_limbs([1, 0, 0, 0]);
    pub const ZERO: Self = Self::from_limbs([0, 0, 0, 0]);

    #[inline(always)]
    pub const fn exp10(n: usize) -> Self {
        match n {
            0 => Self::ONE,
            _ => mul_full_inline_const(&Self::exp10(n - 1),10u32 as u64).0
        }
    }

    #[inline(always)]
    pub const fn is_zero(&self) -> bool {
        self.0[0] == 0 && self.0[1] == 0 && self.0[2] == 0 && self.0[3] == 0
    }

    #[inline(always)]
    pub const fn zero() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub const fn one() -> Self {
        Self::ONE
    }

    // Force inlined because it is a trivial conversion which appears in many hot
    // paths
    #[inline(always)]
    pub const fn from_limbs(limbs: [u64; 4]) -> Self {
        Self(limbs)
    }

    // Force inlined because it is a trivial conversion which appears in many hot
    // paths
    #[inline(always)]
    pub const fn as_limbs(&self) -> &[u64; 4] {
        &self.0
    }

    // It's important that this gets inlined, because `index` is nearly always
    // a compile time constant, which means the range check will get optimized
    // away.
    // TODO: Make const fn
    #[inline(always)]
    pub const fn limb(&self, index: usize) -> u64 {
        self.0.get(index).cloned().unwrap_or_default()
    }

    // It's important that this gets inlined, because `index` is nearly always
    // a compile time constant, which means the range check will get optimized
    // away.
    #[inline(always)]
    pub fn set_limb(&mut self, index: usize, value: u64) {
        if let Some(elem) = self.0.get_mut(index) {
            *elem = value;
        } else {
            panic!("Limb out of range.")
        }
    }

    #[inline(always)]
    pub fn checked_mul(self, other: Self) -> Option<Self> {
        let (result, carry) = self.mul_full_inline(&other);
        if carry.is_zero() {
            Some(result)
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn overflowing_add(self, other: Self) -> (Self, bool) {
        let mut carry = 0u64;
        let (res1, overflow1) = self.0[0].overflowing_add(other.0[0]);
        let (res2, overflow2) = res1.overflowing_add(carry);
        carry = overflow1 as u64 + overflow2 as u64;
        let (res3, overflow3) = self.0[1].overflowing_add(other.0[1]);
        let (res4, overflow4) = res3.overflowing_add(carry);
        carry = overflow3 as u64 + overflow4 as u64;
        let (res5, overflow5) = self.0[2].overflowing_add(other.0[2]);
        let (res6, overflow6) = res5.overflowing_add(carry);
        carry = overflow5 as u64 + overflow6 as u64;
        let (res7, overflow7) = self.0[3].overflowing_add(other.0[3]);
        let (res8, overflow8) = res7.overflowing_add(carry);
        carry = overflow7 as u64 + overflow8 as u64;

        (U256([res2, res4, res6, res8]), carry > 0)
    }
}

impl PartialOrd for U256 {
    // This is a small function that appears often in hot paths.
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U256 {
    // This is a small function that appears often in hot paths.
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        let t = self.limb(3).cmp(&other.limb(3));
        if t != Ordering::Equal {
            return t;
        }
        let t = self.limb(2).cmp(&other.limb(2));
        if t != Ordering::Equal {
            return t;
        }
        let t = self.limb(1).cmp(&other.limb(1));
        if t != Ordering::Equal {
            return t;
        }
        self.limb(0).cmp(&other.limb(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::u256h;

    #[allow(dead_code)]
    const TEST_CONST: U256 =
        u256h!("0800000000000010ffffffffffffffffffffffffffffffffffffffffffffffff");
}
