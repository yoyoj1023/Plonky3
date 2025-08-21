use core::borrow::Borrow;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_baby_bear::{BabyBear, Poseidon2BabyBear};
use p3_challenger::DuplexChallenger;
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_field::{Field, PrimeField64, PrimeCharacteristicRing};
use p3_fri::{TwoAdicFriPcs, create_test_fri_params};
use p3_matrix::{Matrix, dense::RowMajorMatrix};
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};
use p3_uni_stark::{StarkConfig, prove, verify};
use rand::SeedableRng;
use rand::rngs::SmallRng;

/// Fibonacci sequence Chip structure
pub struct FibonacciChip;

/// Number of columns in Fibonacci sequence (two columns: a and b)
const NUM_FIBONACCI_COLS: usize = 2;

/// Represents one row of data in the Fibonacci sequence
pub struct FibonacciRow<F> {
    pub a: F,  // F(i)
    pub b: F,  // F(i+1)
}

impl<F> FibonacciRow<F> {
    const fn new(a: F, b: F) -> Self {
        Self { a, b }
    }
}

/// Implement borrow conversion from slice to FibonacciRow
impl<F> Borrow<FibonacciRow<F>> for [F] {
    fn borrow(&self) -> &FibonacciRow<F> {
        debug_assert_eq!(self.len(), NUM_FIBONACCI_COLS);
        let (prefix, shorts, suffix) = unsafe { self.align_to::<FibonacciRow<F>>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(shorts.len(), 1);
        &shorts[0]
    }
}

/// Implement BaseAir trait for FibonacciChip
impl<F> BaseAir<F> for FibonacciChip {
    fn width(&self) -> usize {
        NUM_FIBONACCI_COLS
    }
}

/// Implement Air trait for FibonacciChip, defining constraints
impl<AB: AirBuilder> Air<AB> for FibonacciChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();

        // Get current row and next row data
        let (local, next) = (
            main.row_slice(0).expect("Matrix is empty?"),
            main.row_slice(1).expect("Matrix only has 1 row?"),
        );
        let local: &FibonacciRow<AB::Var> = (*local).borrow();
        let next: &FibonacciRow<AB::Var> = (*next).borrow();

        // Initial constraints: first row must be a=0, b=1
        let mut when_first_row = builder.when_first_row();
        when_first_row.assert_eq(local.a.clone(), AB::Expr::ZERO);
        when_first_row.assert_one(local.b.clone());

        // Transition constraints: define Fibonacci sequence recurrence relation
        let mut when_transition = builder.when_transition();
        
        // next_a = current_b
        when_transition.assert_eq(next.a.clone(), local.b.clone());
        
        // next_b = current_a + current_b
        when_transition.assert_eq(next.b.clone(), local.a.clone() + local.b.clone());
    }
}

/// Generate execution trace for Fibonacci sequence
impl FibonacciChip {
    pub fn generate_trace<F: PrimeField64>(n: usize) -> RowMajorMatrix<F> {
        assert!(n.is_power_of_two(), "Trace length must be a power of two");

        let mut trace = RowMajorMatrix::new(
            F::zero_vec(n * NUM_FIBONACCI_COLS), 
            NUM_FIBONACCI_COLS
        );

        let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<FibonacciRow<F>>() };
        assert!(prefix.is_empty(), "Alignment should match");
        assert!(suffix.is_empty(), "Alignment should match");
        assert_eq!(rows.len(), n);

        // Set initial values: F(0) = 0, F(1) = 1
        rows[0] = FibonacciRow::new(F::from_u64(0), F::from_u64(1));

        // Calculate subsequent Fibonacci sequence values
        for i in 1..n {
            rows[i].a = rows[i - 1].b;
            rows[i].b = rows[i - 1].a + rows[i - 1].b;
        }

        trace
    }
}

// Type definitions
type Val = BabyBear;
type Perm = Poseidon2BabyBear<16>;
type MyHash = PaddingFreeSponge<Perm, 16, 8, 8>;
type MyCompress = TruncatedPermutation<Perm, 2, 8, 16>;
type ValMmcs = MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 8>;
type Challenge = BinomialExtensionField<Val, 4>;
type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
type Challenger = DuplexChallenger<Val, Perm, 16, 8>;
type Dft = Radix2DitParallel<Val>;
type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;

fn main() {
    println!("🔢 Starting Fibonacci sequence zero-knowledge prover...");

    // Set up random number generator and cryptographic components
    let mut rng = SmallRng::seed_from_u64(42);
    let perm = Perm::new_from_rng_128(&mut rng);
    let hash = MyHash::new(perm.clone());
    let compress = MyCompress::new(perm.clone());
    let val_mmcs = ValMmcs::new(hash, compress);
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
    let dft = Dft::default();

    // Create FRI parameters
    let fri_params = create_test_fri_params(challenge_mmcs, 2);
    let pcs = Pcs::new(dft, val_mmcs, fri_params);
    let challenger = Challenger::new(perm);
    let config = MyConfig::new(pcs, challenger);

    // Create Fibonacci Chip
    let chip = FibonacciChip;

    // Generate execution trace with 1000 rows (must be power of 2)
    let n = 1024; // 2^10 = 1024, closest power of 2 to 1000
    println!("📊 Generating {} rows of Fibonacci sequence execution trace...", n);
    let trace = FibonacciChip::generate_trace::<Val>(n);

    // Display results of first few rows
    println!("✨ First few Fibonacci numbers:");
    for i in 0..10.min(n) {
        let a = trace.get(i, 0).unwrap();
        let b = trace.get(i, 1).unwrap();
        println!("  F({}) = {:?}, F({}) = {:?}", i, a, i + 1, b);
    }

    // Generate proof
    println!("🔐 Generating STARK proof...");
    let proof = prove(&config, &chip, trace, &vec![]);
    println!("✅ Proof generation completed!");

    // Verify proof
    println!("🔍 Verifying proof...");
    match verify(&config, &chip, &proof, &vec![]) {
        Ok(_) => println!("🎉 Proof verification successful! Fibonacci sequence computation correctness has been proven."),
        Err(e) => println!("❌ Proof verification failed: {:?}", e),
    }

    println!("🏁 Fibonacci sequence zero-knowledge prover completed!");
} 
