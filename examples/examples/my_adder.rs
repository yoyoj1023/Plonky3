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

/// Represents an addition instruction
#[derive(Clone, Copy, Debug)]
struct Instruction {
    dest: usize, // Destination register index 0-3
    src1: usize, // Source1 register index 0-3
    src2: usize, // Source2 register index 0-3
}

/// Universal adder processor Chip structure
pub struct AdderChip;

/// Number of columns in universal adder processor (16 columns)
/// 4 register values + 4*3 selectors
const NUM_ADDER_COLS: usize = 16;

/// Represents one row of data in the universal adder processor
/// Contains 4 register values and 12 selectors
#[derive(Clone, Copy)]
pub struct AdderRow<F> {
    // Register values
    pub r0: F,
    pub r1: F,
    pub r2: F,
    pub r3: F,
    // Destination selectors (one-hot)
    pub dest_0: F,
    pub dest_1: F,
    pub dest_2: F,
    pub dest_3: F,
    // Source1 selectors (one-hot)
    pub src1_0: F,
    pub src1_1: F,
    pub src1_2: F,
    pub src1_3: F,
    // Source2 selectors (one-hot)
    pub src2_0: F,
    pub src2_1: F,
    pub src2_2: F,
    pub src2_3: F,
}

/// Implement borrow conversion from slice to AdderRow
impl<F> Borrow<AdderRow<F>> for [F] {
    fn borrow(&self) -> &AdderRow<F> {
        debug_assert_eq!(self.len(), NUM_ADDER_COLS);
        let (prefix, shorts, suffix) = unsafe { self.align_to::<AdderRow<F>>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(shorts.len(), 1);
        &shorts[0]
    }
}

/// Implement BaseAir trait for AdderChip
impl<F> BaseAir<F> for AdderChip {
    fn width(&self) -> usize {
        NUM_ADDER_COLS
    }
}

/// Implement Air trait for AdderChip, defining constraints
impl<AB: AirBuilder> Air<AB> for AdderChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();

        // Get current row and next row data
        let (local, next) = (
            main.row_slice(0).expect("Matrix is empty?"),
            main.row_slice(1).expect("Matrix only has 1 row?"),
        );
        let local: &AdderRow<AB::Var> = (*local).borrow();
        let next: &AdderRow<AB::Var> = (*next).borrow();

        // Constraint 1: Selector Validity
        // Ensure all selector field values are 0 or 1
        builder.assert_bool(local.dest_0.clone());
        builder.assert_bool(local.dest_1.clone());
        builder.assert_bool(local.dest_2.clone());
        builder.assert_bool(local.dest_3.clone());
        
        builder.assert_bool(local.src1_0.clone());
        builder.assert_bool(local.src1_1.clone());
        builder.assert_bool(local.src1_2.clone());
        builder.assert_bool(local.src1_3.clone());
        
        builder.assert_bool(local.src2_0.clone());
        builder.assert_bool(local.src2_1.clone());
        builder.assert_bool(local.src2_2.clone());
        builder.assert_bool(local.src2_3.clone());

        // Ensure each group of selectors is one-hot
        builder.assert_one(
            local.dest_0.clone() + local.dest_1.clone() + local.dest_2.clone() + local.dest_3.clone()
        );
        builder.assert_one(
            local.src1_0.clone() + local.src1_1.clone() + local.src1_2.clone() + local.src1_3.clone()
        );
        builder.assert_one(
            local.src2_0.clone() + local.src2_1.clone() + local.src2_2.clone() + local.src2_3.clone()
        );

        // Constraint 2: State Transition
        let mut when_transition = builder.when_transition();
        
        // Calculate source values (using dot product)
        let src1_val = local.r0.clone() * local.src1_0.clone()
                     + local.r1.clone() * local.src1_1.clone()
                     + local.r2.clone() * local.src1_2.clone()
                     + local.r3.clone() * local.src1_3.clone();

        let src2_val = local.r0.clone() * local.src2_0.clone()
                     + local.r1.clone() * local.src2_1.clone()
                     + local.r2.clone() * local.src2_2.clone()
                     + local.r3.clone() * local.src2_3.clone();

        // Calculate addition result
        let add_result = src1_val + src2_val;

        // Constrain each register's next state
        // If dest_i is 1, then next_reg[i] = add_result
        // If dest_i is 0, then next_reg[i] = local_reg[i]
        let regs = [&local.r0, &local.r1, &local.r2, &local.r3];
        let next_regs = [&next.r0, &next.r1, &next.r2, &next.r3];
        let dest_selectors = [&local.dest_0, &local.dest_1, &local.dest_2, &local.dest_3];

        for i in 0..4 {
            let expected_next = regs[i].clone() * (AB::Expr::ONE - dest_selectors[i].clone()) 
                              + add_result.clone() * dest_selectors[i].clone();
            when_transition.assert_eq(next_regs[i].clone(), expected_next);
        }
    }
}

/// Generate execution trace for universal adder processor
impl AdderChip {
    pub fn generate_trace<F: PrimeField64>(
        program: Vec<Instruction>,
        initial_regs: [F; 4],
    ) -> RowMajorMatrix<F> {
        let n = program.len();
        assert!(n > 0, "Program cannot be empty");
        
        // Ensure length is power of 2 (for Plonky3 compatibility)
        let trace_len = if n.is_power_of_two() { n } else { n.next_power_of_two() };
        
        let mut trace = RowMajorMatrix::new(
            F::zero_vec(trace_len * NUM_ADDER_COLS),
            NUM_ADDER_COLS,
        );

        let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<AdderRow<F>>() };
        assert!(prefix.is_empty(), "Alignment should match");
        assert!(suffix.is_empty(), "Alignment should match");
        assert_eq!(rows.len(), trace_len);

        // Initialize current register state
        let mut current_regs = initial_regs;

        // Process each instruction
        for (i, instruction) in program.iter().enumerate() {
            // Create current row
            let mut row = AdderRow {
                r0: current_regs[0],
                r1: current_regs[1],
                r2: current_regs[2],
                r3: current_regs[3],
                dest_0: F::from_u64(0),
                dest_1: F::from_u64(0),
                dest_2: F::from_u64(0),
                dest_3: F::from_u64(0),
                src1_0: F::from_u64(0),
                src1_1: F::from_u64(0),
                src1_2: F::from_u64(0),
                src1_3: F::from_u64(0),
                src2_0: F::from_u64(0),
                src2_1: F::from_u64(0),
                src2_2: F::from_u64(0),
                src2_3: F::from_u64(0),
            };

            // Set selectors (one-hot encoding)
            match instruction.dest {
                0 => row.dest_0 = F::from_u64(1),
                1 => row.dest_1 = F::from_u64(1),
                2 => row.dest_2 = F::from_u64(1),
                3 => row.dest_3 = F::from_u64(1),
                _ => panic!("Invalid dest register: {}", instruction.dest),
            }

            match instruction.src1 {
                0 => row.src1_0 = F::from_u64(1),
                1 => row.src1_1 = F::from_u64(1),
                2 => row.src1_2 = F::from_u64(1),
                3 => row.src1_3 = F::from_u64(1),
                _ => panic!("Invalid src1 register: {}", instruction.src1),
            }

            match instruction.src2 {
                0 => row.src2_0 = F::from_u64(1),
                1 => row.src2_1 = F::from_u64(1),
                2 => row.src2_2 = F::from_u64(1),
                3 => row.src2_3 = F::from_u64(1),
                _ => panic!("Invalid src2 register: {}", instruction.src2),
            }

            // Add row to trace
            rows[i] = row;

            // Update register state (prepare for next row)
            let src1_val = current_regs[instruction.src1];
            let src2_val = current_regs[instruction.src2];
            let add_result = src1_val + src2_val;
            current_regs[instruction.dest] = add_result;
        }

        // If padding additional rows is needed (maintain last state)
        for i in n..trace_len {
            rows[i] = rows[n - 1];
        }

        trace
    }
}

// Type definitions (same as Fibonacci example)
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
    println!("🔧 Starting universal adder processor zero-knowledge prover...");

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

    // Create universal adder processor Chip
    let chip = AdderChip;

    // Define test program (add more instructions to meet STARK minimum size requirement)
    println!("📝 Defining test program:");
    let mut program = vec![
        Instruction { dest: 2, src1: 0, src2: 1 }, // r2 = r0 + r1   (1 + 2 = 3)
        Instruction { dest: 3, src1: 2, src2: 2 }, // r3 = r2 + r2   (3 + 3 = 6)
        Instruction { dest: 0, src1: 3, src2: 1 }, // r0 = r3 + r1   (6 + 2 = 8)
    ];
    
    // Add more instructions to reach 2^6 = 64 instructions
    while program.len() < 64 {
        program.push(Instruction { dest: 1, src1: 1, src2: 1 }); // r1 = r1 + r1 (doubling)
        if program.len() < 64 {
            program.push(Instruction { dest: 2, src1: 2, src2: 0 }); // r2 = r2 + r0
        }
        if program.len() < 64 {
            program.push(Instruction { dest: 3, src1: 3, src2: 1 }); // r3 = r3 + r1
        }
        if program.len() < 64 {
            program.push(Instruction { dest: 0, src1: 0, src2: 2 }); // r0 = r0 + r2
        }
    }

    // Display only first few important instructions
    for (i, inst) in program.iter().take(10).enumerate() {
        println!("  Instruction {}: r{} = r{} + r{}", i + 1, inst.dest, inst.src1, inst.src2);
    }
    if program.len() > 10 {
        println!("  ... and {} more instructions", program.len() - 10);
    }

    // Set initial register state
    let initial_regs = [Val::from_u64(1), Val::from_u64(2), Val::from_u64(0), Val::from_u64(0)];
    println!("🏁 Initial register state: [r0={}, r1={}, r2={}, r3={}]", 
             initial_regs[0].as_canonical_u64(),
             initial_regs[1].as_canonical_u64(), 
             initial_regs[2].as_canonical_u64(),
             initial_regs[3].as_canonical_u64());

    // Generate execution trace
    println!("📊 Generating execution trace...");
    let trace = AdderChip::generate_trace::<Val>(program.clone(), initial_regs);

    // Manual verification of execution results (display only first few steps and final result)
    println!("✨ Program execution process:");
    let mut regs = [1u64, 2u64, 0u64, 0u64];
    println!("  Initial state: [r0={}, r1={}, r2={}, r3={}]", regs[0], regs[1], regs[2], regs[3]);
    
    for (i, inst) in program.iter().enumerate() {
        let src1_val = regs[inst.src1];
        let src2_val = regs[inst.src2];
        let result = src1_val + src2_val;
        regs[inst.dest] = result;
        
        // Display detailed execution process for only first 5 steps
        if i < 5 {
            println!("  Execute instruction {}: r{} = r{} + r{} = {} + {} = {} -> [r0={}, r1={}, r2={}, r3={}]", 
                     i + 1, inst.dest, inst.src1, inst.src2, src1_val, src2_val, result,
                     regs[0], regs[1], regs[2], regs[3]);
        }
    }
    
    if program.len() > 5 {
        println!("  ... executed {} more instructions", program.len() - 5);
    }

    println!("🎯 Final register state: [r0={}, r1={}, r2={}, r3={}]", regs[0], regs[1], regs[2], regs[3]);

    // Generate proof
    println!("🔐 Generating STARK proof...");
    let proof = prove(&config, &chip, trace, &vec![]);
    println!("✅ Proof generation completed!");

    // Verify proof
    println!("🔍 Verifying proof...");
    match verify(&config, &chip, &proof, &vec![]) {
        Ok(_) => println!("🎉 Proof verification successful! Universal adder processor execution correctness has been proven."),
        Err(e) => println!("❌ Proof verification failed: {:?}", e),
    }

    println!("🏁 Universal adder processor zero-knowledge prover completed!");
} 
