use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use core::{array, fmt};

use num_bigint::BigUint;
use p3_field::exponentiation::exp_10540996611094048183;
use p3_field::integers::QuotientMap;
use p3_field::op_assign_macros::{
    impl_add_assign, impl_div_methods, impl_mul_methods, impl_sub_assign,
};
use p3_field::{
    Field, InjectiveMonomial, Packable, PermutationMonomial, PrimeCharacteristicRing, PrimeField,
    PrimeField64, RawDataSerializable, TwoAdicField, halve_u64, impl_raw_serializable_primefield64,
    quotient_map_large_iint, quotient_map_large_uint, quotient_map_small_int,
};
use p3_util::{assume, branch_hint, flatten_to_base, gcd_inner};
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use serde::{Deserialize, Serialize};

/// The Goldilocks prime
const P: u64 = 0xFFFF_FFFF_0000_0001;

/// The prime field known as Goldilocks, defined as `F_p` where `p = 2^64 - 2^32 + 1`.
///
/// Note that the safety of deriving `Serialize` and `Deserialize` relies on the fact that the internal value can be any u64.
#[derive(Copy, Clone, Default, Serialize, Deserialize)]
#[repr(transparent)] // Important for reasoning about memory layout
pub struct Goldilocks {
    /// Not necessarily canonical.
    pub(crate) value: u64,
}

impl Goldilocks {
    pub(crate) const fn new(value: u64) -> Self {
        Self { value }
    }

    /// Convert a constant u64 array into a constant Goldilocks array.
    ///
    /// This is a const version of `.map(Goldilocks::new)`.
    #[inline]
    #[must_use]
    pub(crate) const fn new_array<const N: usize>(input: [u64; N]) -> [Self; N] {
        let mut output = [Self::ZERO; N];
        let mut i = 0;
        while i < N {
            output[i].value = input[i];
            i += 1;
        }
        output
    }

    /// Two's complement of `ORDER`, i.e. `2^64 - ORDER = 2^32 - 1`.
    const NEG_ORDER: u64 = Self::ORDER_U64.wrapping_neg();

    /// A list of generators for the two-adic subgroups of the goldilocks field.
    ///
    /// These satisfy the properties that `TWO_ADIC_GENERATORS[0] = 1` and `TWO_ADIC_GENERATORS[i+1]^2 = TWO_ADIC_GENERATORS[i]`.
    pub const TWO_ADIC_GENERATORS: [Goldilocks; 33] = Goldilocks::new_array([
        0x0000000000000001,
        0xffffffff00000000,
        0x0001000000000000,
        0xfffffffeff000001,
        0xefffffff00000001,
        0x00003fffffffc000,
        0x0000008000000000,
        0xf80007ff08000001,
        0xbf79143ce60ca966,
        0x1905d02a5c411f4e,
        0x9d8f2ad78bfed972,
        0x0653b4801da1c8cf,
        0xf2c35199959dfcb6,
        0x1544ef2335d17997,
        0xe0ee099310bba1e2,
        0xf6b2cffe2306baac,
        0x54df9630bf79450e,
        0xabd0a6e8aa3d8a0e,
        0x81281a7b05f9beac,
        0xfbd41c6b8caa3302,
        0x30ba2ecd5e93e76d,
        0xf502aef532322654,
        0x4b2a18ade67246b5,
        0xea9d5a1336fbc98b,
        0x86cdcc31c307e171,
        0x4bbaf5976ecfefd8,
        0xed41d05b78d6e286,
        0x10d78dd8915a171d,
        0x59049500004a4485,
        0xdfa8c93ba46d2666,
        0x7e9bd009b86a0845,
        0x400a7f755588e659,
        0x185629dcda58878c,
    ]);

    /// A list of powers of two from 0 to 95.
    ///
    /// Note that 2^{96} = -1 mod P so all powers of two can be simply
    /// derived from this list.
    const POWERS_OF_TWO: [Self; 96] = {
        let mut powers_of_two = [Goldilocks::ONE; 96];

        let mut i = 1;
        while i < 64 {
            powers_of_two[i] = Goldilocks::new(1 << i);
            i += 1;
        }
        let mut var = Goldilocks::new(1 << 63);
        while i < 96 {
            var = const_add(var, var);
            powers_of_two[i] = var;
            i += 1;
        }
        powers_of_two
    };
}

impl PartialEq for Goldilocks {
    fn eq(&self, other: &Self) -> bool {
        self.as_canonical_u64() == other.as_canonical_u64()
    }
}

impl Eq for Goldilocks {}

impl Packable for Goldilocks {}

impl Hash for Goldilocks {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_canonical_u64());
    }
}

impl Ord for Goldilocks {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_canonical_u64().cmp(&other.as_canonical_u64())
    }
}

impl PartialOrd for Goldilocks {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Goldilocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.as_canonical_u64(), f)
    }
}

impl Debug for Goldilocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.as_canonical_u64(), f)
    }
}

impl Distribution<Goldilocks> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Goldilocks {
        loop {
            let next_u64 = rng.next_u64();
            let is_canonical = next_u64 < Goldilocks::ORDER_U64;
            if is_canonical {
                return Goldilocks::new(next_u64);
            }
        }
    }
}

impl PrimeCharacteristicRing for Goldilocks {
    type PrimeSubfield = Self;

    const ZERO: Self = Self::new(0);
    const ONE: Self = Self::new(1);
    const TWO: Self = Self::new(2);
    const NEG_ONE: Self = Self::new(Self::ORDER_U64 - 1);

    #[inline]
    fn from_prime_subfield(f: Self::PrimeSubfield) -> Self {
        f
    }

    #[inline]
    fn from_bool(b: bool) -> Self {
        Self::new(b.into())
    }

    #[inline]
    fn mul_2exp_u64(&self, exp: u64) -> Self {
        // In the Goldilocks field, 2^96 = -1 mod P and 2^192 = 1 mod P.
        if exp < 96 {
            *self * Self::POWERS_OF_TWO[exp as usize]
        } else if exp < 192 {
            -*self * Self::POWERS_OF_TWO[(exp - 96) as usize]
        } else {
            self.mul_2exp_u64(exp % 192)
        }
    }

    #[inline]
    fn sum_array<const N: usize>(input: &[Self]) -> Self {
        assert_eq!(N, input.len());
        // Benchmarking shows that for N <= 3 it's faster to sum the elements directly
        // but for N > 3 it's faster to use the .sum() methods which passes through u128's
        // allowing for delayed reductions.
        match N {
            0 => Self::ZERO,
            1 => input[0],
            2 => input[0] + input[1],
            3 => input[0] + input[1] + input[2],
            _ => input.iter().copied().sum(),
        }
    }

    #[inline]
    fn dot_product<const N: usize>(lhs: &[Self; N], rhs: &[Self; N]) -> Self {
        // The constant OFFSET has 2 important properties:
        // 1. It is a multiple of P.
        // 2. It is greater than the maximum possible value of the sum of the products of two u64s.
        const OFFSET: u128 = ((P as u128) << 64) - (P as u128) + ((P as u128) << 32);
        assert!((N as u32) <= (1 << 31));
        match N {
            0 => Self::ZERO,
            1 => lhs[0] * rhs[0],
            2 => {
                // We unroll the N = 2 case as it is slightly faster and this is an important case
                // as a major use is in extension field arithmetic and Goldilocks has a degree 2 extension.
                let long_prod_0 = (lhs[0].value as u128) * (rhs[0].value as u128);
                let long_prod_1 = (lhs[1].value as u128) * (rhs[1].value as u128);

                // We know that long_prod_0, long_prod_1 < OFFSET.
                // Thus if long_prod_0 + long_prod_1 overflows, we can just subtract OFFSET.
                let (sum, over) = long_prod_0.overflowing_add(long_prod_1);
                // Compiler really likes defining sum_corr here instead of in the if/else.
                let sum_corr = sum.wrapping_sub(OFFSET);
                if over {
                    reduce128(sum_corr)
                } else {
                    reduce128(sum)
                }
            }
            _ => {
                let (lo_plus_hi, hi) = lhs
                    .iter()
                    .zip(rhs)
                    .map(|(x, y)| (x.value as u128) * (y.value as u128))
                    .fold((0_u128, 0_u64), |(acc_lo, acc_hi), val| {
                        // Split val into (hi, lo) where hi is the upper 32 bits and lo is the lower 96 bits.
                        let val_hi = (val >> 96) as u64;
                        // acc_hi accumulates hi, acc_lo accumulates lo + 2^{96}hi.
                        // As N <= 2^32, acc_hi cannot overflow.
                        unsafe { (acc_lo.wrapping_add(val), acc_hi.unchecked_add(val_hi)) }
                    });
                // First, remove the hi part from lo_plus_hi.
                let lo = lo_plus_hi.wrapping_sub((hi as u128) << 96);
                // As 2^{96} = -1 mod P, we simply need to reduce lo - hi.
                // As N <= 2^31, lo < 2^127 and hi < 2^63 < P. Hence the equation below will not over or underflow.
                let sum = unsafe { lo.unchecked_add(P.unchecked_sub(hi) as u128) };
                reduce128(sum)
            }
        }
    }

    #[inline]
    fn zero_vec(len: usize) -> Vec<Self> {
        // SAFETY:
        // Due to `#[repr(transparent)]`, Goldilocks and u64 have the same size, alignment
        // and memory layout making `flatten_to_base` safe. This this will create
        // a vector Goldilocks elements with value set to 0.
        unsafe { flatten_to_base(vec![0u64; len]) }
    }
}

/// Degree of the smallest permutation polynomial for Goldilocks.
///
/// As p - 1 = 2^32 * 3 * 5 * 17 * ... the smallest choice for a degree D satisfying gcd(p - 1, D) = 1 is 7.
impl InjectiveMonomial<7> for Goldilocks {}

impl PermutationMonomial<7> for Goldilocks {
    /// In the field `Goldilocks`, `a^{1/7}` is equal to a^{10540996611094048183}.
    ///
    /// This follows from the calculation `7*10540996611094048183 = 4*(2^64 - 2**32) + 1 = 1 mod (p - 1)`.
    fn injective_exp_root_n(&self) -> Self {
        exp_10540996611094048183(*self)
    }
}

impl RawDataSerializable for Goldilocks {
    impl_raw_serializable_primefield64!();
}

impl Field for Goldilocks {
    // TODO: Add cfg-guarded Packing for NEON

    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx2",
        not(all(feature = "nightly-features", target_feature = "avx512f"))
    ))]
    type Packing = crate::PackedGoldilocksAVX2;

    #[cfg(all(
        feature = "nightly-features",
        target_arch = "x86_64",
        target_feature = "avx512f"
    ))]
    type Packing = crate::PackedGoldilocksAVX512;
    #[cfg(not(any(
        all(
            target_arch = "x86_64",
            target_feature = "avx2",
            not(all(feature = "nightly-features", target_feature = "avx512f"))
        ),
        all(
            feature = "nightly-features",
            target_arch = "x86_64",
            target_feature = "avx512f"
        ),
    )))]
    type Packing = Self;

    // Sage: GF(2^64 - 2^32 + 1).multiplicative_generator()
    const GENERATOR: Self = Self::new(7);

    fn is_zero(&self) -> bool {
        self.value == 0 || self.value == Self::ORDER_U64
    }

    fn try_inverse(&self) -> Option<Self> {
        if self.is_zero() {
            return None;
        }

        Some(gcd_inversion(*self))
    }

    #[inline]
    fn halve(&self) -> Self {
        Self::new(halve_u64::<P>(self.value))
    }

    #[inline]
    fn div_2exp_u64(&self, mut exp: u64) -> Self {
        // In the goldilocks field, 2^192 = 1 mod P.
        // Thus 2^{-n} = 2^{192 - n} mod P.
        exp %= 192;
        self.mul_2exp_u64(192 - exp)
    }

    #[inline]
    fn order() -> BigUint {
        P.into()
    }
}

// We use macros to implement QuotientMap<Int> for all integer types except for u64 and i64.
quotient_map_small_int!(Goldilocks, u64, [u8, u16, u32]);
quotient_map_small_int!(Goldilocks, i64, [i8, i16, i32]);
quotient_map_large_uint!(
    Goldilocks,
    u64,
    Goldilocks::ORDER_U64,
    "`[0, 2^64 - 2^32]`",
    "`[0, 2^64 - 1]`",
    [u128]
);
quotient_map_large_iint!(
    Goldilocks,
    i64,
    "`[-(2^63 - 2^31), 2^63 - 2^31]`",
    "`[1 + 2^32 - 2^64, 2^64 - 1]`",
    [(i128, u128)]
);

impl QuotientMap<u64> for Goldilocks {
    /// Convert a given `u64` integer into an element of the `Goldilocks` field.
    ///
    /// No reduction is needed as the internal value is allowed
    /// to be any u64.
    #[inline]
    fn from_int(int: u64) -> Self {
        Self::new(int)
    }

    /// Convert a given `u64` integer into an element of the `Goldilocks` field.
    ///
    /// Return `None` if the given integer is greater than `p = 2^64 - 2^32 + 1`.
    #[inline]
    fn from_canonical_checked(int: u64) -> Option<Self> {
        (int < Self::ORDER_U64).then(|| Self::new(int))
    }

    /// Convert a given `u64` integer into an element of the `Goldilocks` field.
    ///
    /// # Safety
    /// In this case this function is actually always safe as the internal
    /// value is allowed to be any u64.
    #[inline(always)]
    unsafe fn from_canonical_unchecked(int: u64) -> Self {
        Self::new(int)
    }
}

impl QuotientMap<i64> for Goldilocks {
    /// Convert a given `i64` integer into an element of the `Goldilocks` field.
    ///
    /// We simply need to deal with the sign.
    #[inline]
    fn from_int(int: i64) -> Self {
        if int >= 0 {
            Self::new(int as u64)
        } else {
            Self::new(Self::ORDER_U64.wrapping_add_signed(int))
        }
    }

    /// Convert a given `i64` integer into an element of the `Goldilocks` field.
    ///
    /// Returns none if the input does not lie in the range `(-(2^63 - 2^31), 2^63 - 2^31)`.
    #[inline]
    fn from_canonical_checked(int: i64) -> Option<Self> {
        const POS_BOUND: i64 = (P >> 1) as i64;
        const NEG_BOUND: i64 = -POS_BOUND;
        match int {
            0..=POS_BOUND => Some(Self::new(int as u64)),
            NEG_BOUND..0 => Some(Self::new(Self::ORDER_U64.wrapping_add_signed(int))),
            _ => None,
        }
    }

    /// Convert a given `i64` integer into an element of the `Goldilocks` field.
    ///
    /// # Safety
    /// In this case this function is actually always safe as the internal
    /// value is allowed to be any u64.
    #[inline(always)]
    unsafe fn from_canonical_unchecked(int: i64) -> Self {
        Self::from_int(int)
    }
}

impl PrimeField for Goldilocks {
    fn as_canonical_biguint(&self) -> BigUint {
        self.as_canonical_u64().into()
    }
}

impl PrimeField64 for Goldilocks {
    const ORDER_U64: u64 = P;

    #[inline]
    fn as_canonical_u64(&self) -> u64 {
        let mut c = self.value;
        // We only need one condition subtraction, since 2 * ORDER would not fit in a u64.
        if c >= Self::ORDER_U64 {
            c -= Self::ORDER_U64;
        }
        c
    }
}

impl TwoAdicField for Goldilocks {
    const TWO_ADICITY: usize = 32;

    fn two_adic_generator(bits: usize) -> Self {
        assert!(bits <= Self::TWO_ADICITY);
        Self::TWO_ADIC_GENERATORS[bits]
    }
}

/// A const version of the addition function.
///
/// Useful for constructing constants values in const contexts. Outside of
/// const contexts, Add should be used instead.
#[inline]
const fn const_add(lhs: Goldilocks, rhs: Goldilocks) -> Goldilocks {
    let (sum, over) = lhs.value.overflowing_add(rhs.value);
    let (mut sum, over) = sum.overflowing_add((over as u64) * Goldilocks::NEG_ORDER);
    if over {
        sum += Goldilocks::NEG_ORDER;
    }
    Goldilocks::new(sum)
}

impl Add for Goldilocks {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        let (sum, over) = self.value.overflowing_add(rhs.value);
        let (mut sum, over) = sum.overflowing_add(u64::from(over) * Self::NEG_ORDER);
        if over {
            // NB: self.value > Self::ORDER && rhs.value > Self::ORDER is necessary but not
            // sufficient for double-overflow.
            // This assume does two things:
            //  1. If compiler knows that either self.value or rhs.value <= ORDER, then it can skip
            //     this check.
            //  2. Hints to the compiler how rare this double-overflow is (thus handled better with
            //     a branch).
            assume(self.value > Self::ORDER_U64 && rhs.value > Self::ORDER_U64);
            branch_hint();
            sum += Self::NEG_ORDER; // Cannot overflow.
        }
        Self::new(sum)
    }
}

impl Sub for Goldilocks {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        let (diff, under) = self.value.overflowing_sub(rhs.value);
        let (mut diff, under) = diff.overflowing_sub(u64::from(under) * Self::NEG_ORDER);
        if under {
            // NB: self.value < NEG_ORDER - 1 && rhs.value > ORDER is necessary but not
            // sufficient for double-underflow.
            // This assume does two things:
            //  1. If compiler knows that either self.value >= NEG_ORDER - 1 or rhs.value <= ORDER,
            //     then it can skip this check.
            //  2. Hints to the compiler how rare this double-underflow is (thus handled better
            //     with a branch).
            assume(self.value < Self::NEG_ORDER - 1 && rhs.value > Self::ORDER_U64);
            branch_hint();
            diff -= Self::NEG_ORDER; // Cannot underflow.
        }
        Self::new(diff)
    }
}

impl Neg for Goldilocks {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(Self::ORDER_U64 - self.as_canonical_u64())
    }
}

impl Mul for Goldilocks {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        reduce128(u128::from(self.value) * u128::from(rhs.value))
    }
}

impl_add_assign!(Goldilocks);
impl_sub_assign!(Goldilocks);
impl_mul_methods!(Goldilocks);
impl_div_methods!(Goldilocks, Goldilocks);

impl Sum for Goldilocks {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        // This is faster than iter.reduce(|x, y| x + y).unwrap_or(Self::ZERO) for iterators of length > 2.

        // This sum will not overflow so long as iter.len() < 2^64.
        let sum = iter.map(|x| x.value as u128).sum::<u128>();
        reduce128(sum)
    }
}

/// Reduces to a 64-bit value. The result might not be in canonical form; it could be in between the
/// field order and `2^64`.
#[inline]
pub(crate) fn reduce128(x: u128) -> Goldilocks {
    let (x_lo, x_hi) = split(x); // This is a no-op
    let x_hi_hi = x_hi >> 32;
    let x_hi_lo = x_hi & Goldilocks::NEG_ORDER;

    let (mut t0, borrow) = x_lo.overflowing_sub(x_hi_hi);
    if borrow {
        branch_hint(); // A borrow is exceedingly rare. It is faster to branch.
        t0 -= Goldilocks::NEG_ORDER; // Cannot underflow.
    }
    let t1 = x_hi_lo * Goldilocks::NEG_ORDER;
    let t2 = unsafe { add_no_canonicalize_trashing_input(t0, t1) };
    Goldilocks::new(t2)
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
const fn split(x: u128) -> (u64, u64) {
    (x as u64, (x >> 64) as u64)
}

/// Fast addition modulo ORDER for x86-64.
/// This function is marked unsafe for the following reasons:
///   - It is only correct if x + y < 2**64 + ORDER = 0x1ffffffff00000001.
///   - It is only faster in some circumstances. In particular, on x86 it overwrites both inputs in
///     the registers, so its use is not recommended when either input will be used again.
#[inline(always)]
#[cfg(target_arch = "x86_64")]
unsafe fn add_no_canonicalize_trashing_input(x: u64, y: u64) -> u64 {
    unsafe {
        let res_wrapped: u64;
        let adjustment: u64;
        core::arch::asm!(
            "add {0}, {1}",
            // Trick. The carry flag is set iff the addition overflowed.
            // sbb x, y does x := x - y - CF. In our case, x and y are both {1:e}, so it simply does
            // {1:e} := 0xffffffff on overflow and {1:e} := 0 otherwise. {1:e} is the low 32 bits of
            // {1}; the high 32-bits are zeroed on write. In the end, we end up with 0xffffffff in {1}
            // on overflow; this happens be NEG_ORDER.
            // Note that the CPU does not realize that the result of sbb x, x does not actually depend
            // on x. We must write the result to a register that we know to be ready. We have a
            // dependency on {1} anyway, so let's use it.
            "sbb {1:e}, {1:e}",
            inlateout(reg) x => res_wrapped,
            inlateout(reg) y => adjustment,
            options(pure, nomem, nostack),
        );
        assume(x != 0 || (res_wrapped == y && adjustment == 0));
        assume(y != 0 || (res_wrapped == x && adjustment == 0));
        // Add NEG_ORDER == subtract ORDER.
        // Cannot overflow unless the assumption if x + y < 2**64 + ORDER is incorrect.
        res_wrapped + adjustment
    }
}

#[inline(always)]
#[cfg(not(target_arch = "x86_64"))]
unsafe fn add_no_canonicalize_trashing_input(x: u64, y: u64) -> u64 {
    let (res_wrapped, carry) = x.overflowing_add(y);
    // Below cannot overflow unless the assumption if x + y < 2**64 + ORDER is incorrect.
    res_wrapped + Goldilocks::NEG_ORDER * u64::from(carry)
}

/// Compute the inverse of a Goldilocks element `a` using the binary GCD algorithm.
///
/// Instead of applying the standard algorithm this uses a variant inspired by https://eprint.iacr.org/2020/972.pdf.
/// The key idea is to compute update factors which are incorrect by a known power of 2 which
/// can be corrected at the end. These update factors can then be used to construct the inverse
/// via a simple linear combination.
///
/// This is much faster than the standard algorithm as we avoid most of the (more expensive) field arithmetic.
fn gcd_inversion(input: Goldilocks) -> Goldilocks {
    // Initialise our values to the value we want to invert and the prime.
    let (mut a, mut b) = (input.value, P);

    // As the goldilocks prime is 64 bit, initially `len(a) + len(b) ≤ 2 * 64 = 128`.
    // This means we will need `126` iterations of the inner loop ensure `len(a) + len(b) ≤ 2`.
    // We split the iterations into 2 rounds of length 63.
    const ROUND_SIZE: usize = 63;

    // In theory we could make this slightly faster by replacing the first `gcd_inner` by a copy-pasted
    // version which doesn't do any computations involving g. But either the compiler works this out
    // for itself or the speed up is negligible as I couldn't notice any difference in benchmarks.
    let (f00, _, f10, _) = gcd_inner::<ROUND_SIZE>(&mut a, &mut b);
    let (_, _, f11, g11) = gcd_inner::<ROUND_SIZE>(&mut a, &mut b);

    // The update factors are i64's except we need to interpret -2^63 as 2^63.
    // This is because the outputs of `gcd_inner` are always in the range `(-2^ROUND_SIZE, 2^ROUND_SIZE]`.
    let u = from_unusual_int(f00);
    let v = from_unusual_int(f10);
    let u_fac11 = from_unusual_int(f11);
    let v_fac11 = from_unusual_int(g11);

    // Each iteration introduced a factor of 2 and so we need to divide by 2^{126}.
    // But 2^{192} = 1 mod P, so we can instead multiply by 2^{66} as 192 - 126 = 66.
    (u * u_fac11 + v * v_fac11).mul_2exp_u64(66)
}

/// Convert from an i64 to a Goldilocks element but interpret -2^63 as 2^63.
fn from_unusual_int(int: i64) -> Goldilocks {
    if (int >= 0) || (int == i64::MIN) {
        Goldilocks::new(int as u64)
    } else {
        Goldilocks::new(Goldilocks::ORDER_U64.wrapping_add_signed(int))
    }
}

#[cfg(test)]
mod tests {
    use p3_field::extension::BinomialExtensionField;
    use p3_field_testing::{
        test_field, test_field_dft, test_prime_field, test_prime_field_64, test_two_adic_field,
    };

    use super::*;

    type F = Goldilocks;
    type EF = BinomialExtensionField<F, 5>;

    #[test]
    fn test_goldilocks() {
        let f = F::new(100);
        assert_eq!(f.as_canonical_u64(), 100);

        // Over the Goldilocks field, the following set of equations hold
        // p               = 0
        // 2^64 - 2^32 + 1 = 0
        // 2^64            = 2^32 - 1
        let f = F::new(u64::MAX);
        assert_eq!(f.as_canonical_u64(), u32::MAX as u64 - 1);

        let f = F::from_u64(u64::MAX);
        assert_eq!(f.as_canonical_u64(), u32::MAX as u64 - 1);

        // Generator check
        let expected_multiplicative_group_generator = F::new(7);
        assert_eq!(F::GENERATOR, expected_multiplicative_group_generator);
        assert_eq!(F::GENERATOR.as_canonical_u64(), 7_u64);

        // Check on `reduce_u128`
        let x = u128::MAX;
        let y = reduce128(x);
        // The following equality sequence holds, modulo p = 2^64 - 2^32 + 1
        // 2^128 - 1 = (2^64 - 1) * (2^64 + 1)
        //           = (2^32 - 1 - 1) * (2^32 - 1 + 1)
        //           = (2^32 - 2) * (2^32)
        //           = 2^64 - 2 * 2^32
        //           = 2^64 - 2^33
        //           = 2^32 - 1 - 2^33
        //           = - 2^32 - 1
        let expected_result = -F::TWO.exp_power_of_2(5) - F::ONE;
        assert_eq!(y, expected_result);

        let f = F::new(100);
        assert_eq!(f.injective_exp_n().injective_exp_root_n(), f);
        assert_eq!(y.injective_exp_n().injective_exp_root_n(), y);
        assert_eq!(F::TWO.injective_exp_n().injective_exp_root_n(), F::TWO);
    }

    // Goldilocks has a redundant representation for both 0 and 1.
    const ZEROS: [Goldilocks; 2] = [Goldilocks::ZERO, Goldilocks::new(P)];
    const ONES: [Goldilocks; 2] = [Goldilocks::ONE, Goldilocks::new(P + 1)];

    // Get the prime factorization of the order of the multiplicative group.
    // i.e. the prime factorization of P - 1.
    fn multiplicative_group_prime_factorization() -> [(BigUint, u32); 6] {
        [
            (BigUint::from(2u8), 32),
            (BigUint::from(3u8), 1),
            (BigUint::from(5u8), 1),
            (BigUint::from(17u8), 1),
            (BigUint::from(257u16), 1),
            (BigUint::from(65537u32), 1),
        ]
    }

    test_field!(
        crate::Goldilocks,
        &super::ZEROS,
        &super::ONES,
        &super::multiplicative_group_prime_factorization()
    );
    test_prime_field!(crate::Goldilocks);
    test_prime_field_64!(crate::Goldilocks, &super::ZEROS, &super::ONES);
    test_two_adic_field!(crate::Goldilocks);

    test_field_dft!(
        radix2dit,
        crate::Goldilocks,
        super::EF,
        p3_dft::Radix2Dit<_>
    );
    test_field_dft!(bowers, crate::Goldilocks, super::EF, p3_dft::Radix2Bowers);
    test_field_dft!(
        parallel,
        crate::Goldilocks,
        super::EF,
        p3_dft::Radix2DitParallel<crate::Goldilocks>
    );
}
