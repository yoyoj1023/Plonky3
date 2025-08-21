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

/// Operation types: supports addition and subtraction
#[derive(Clone, Copy, Debug, PartialEq)]
enum Opcode {
    ADD,
    SUB,
}

/// Represents an ALU instruction
#[derive(Clone, Copy, Debug)]
struct Instruction {
    op: Opcode,     // Operation type
    dest: usize,    // Destination register index 0-3
    src1: usize,    // Source1 register index 0-3
    src2: usize,    // Source2 register index 0-3
}

/// Universal Arithmetic Logic Unit Chip structure
pub struct AluChip;

/// Number of ALU columns (18 columns)
/// 4 register values + 4*3 register selectors + 2 operation selectors
const NUM_ALU_COLS: usize = 18;

/// Represents one row of ALU data
/// Contains 4 register values, 12 register selectors, and 2 operation selectors
#[derive(Clone, Copy)]
pub struct AluRow<F> {
    // Register values (4 columns)
    pub r0: F,
    pub r1: F,
    pub r2: F,
    pub r3: F,
    // Destination selectors (one-hot, 4 columns)
    pub dest_0: F,
    pub dest_1: F,
    pub dest_2: F,
    pub dest_3: F,
    // Source1 selectors (one-hot, 4 columns)
    pub src1_0: F,
    pub src1_1: F,
    pub src1_2: F,
    pub src1_3: F,
    // Source2 selectors (one-hot, 4 columns)
    pub src2_0: F,
    pub src2_1: F,
    pub src2_2: F,
    pub src2_3: F,
    // Operation selectors (one-hot, 2 columns)
    pub op_add: F,
    pub op_sub: F,
}

/// Implement borrow conversion from slice to AluRow
impl<F> Borrow<AluRow<F>> for [F] {
    fn borrow(&self) -> &AluRow<F> {
        debug_assert_eq!(self.len(), NUM_ALU_COLS);
        let (prefix, shorts, suffix) = unsafe { self.align_to::<AluRow<F>>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(shorts.len(), 1);
        &shorts[0]
    }
}

/// Implement BaseAir trait for AluChip
impl<F> BaseAir<F> for AluChip {
    fn width(&self) -> usize {
        NUM_ALU_COLS
    }
}

/// Implement Air trait for AluChip, defining constraints
impl<AB: AirBuilder> Air<AB> for AluChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();

        // Get current row and next row data
        let (local, next) = (
            main.row_slice(0).expect("Matrix is empty?"),
            main.row_slice(1).expect("Matrix only has 1 row?"),
        );
        let local: &AluRow<AB::Var> = (*local).borrow();
        let next: &AluRow<AB::Var> = (*next).borrow();

        // Constraint 1: Selector Validity
        
        // Register selectors must be 0 or 1
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

        // Operation selectors must be 0 or 1
        builder.assert_bool(local.op_add.clone());
        builder.assert_bool(local.op_sub.clone());

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
        
        // Operation selectors must also be one-hot
        builder.assert_one(local.op_add.clone() + local.op_sub.clone());

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

        // 🔥 Core: Conditional operation result calculation
        // Choose addition or subtraction based on operation selectors
        let add_result = src1_val.clone() + src2_val.clone();
        let sub_result = src1_val - src2_val;
        
        // Use selectors for conditional result selection
        // result = add_result * op_add + sub_result * op_sub
        // When op_add=1, op_sub=0: result = add_result
        // When op_add=0, op_sub=1: result = sub_result
        let result = add_result * local.op_add.clone() + sub_result * local.op_sub.clone();

        // Constrain each register's next state
        // If dest_i is 1, then next_reg[i] = result
        // If dest_i is 0, then next_reg[i] = local_reg[i]
        let regs = [&local.r0, &local.r1, &local.r2, &local.r3];
        let next_regs = [&next.r0, &next.r1, &next.r2, &next.r3];
        let dest_selectors = [&local.dest_0, &local.dest_1, &local.dest_2, &local.dest_3];

        for i in 0..4 {
            let expected_next = regs[i].clone() * (AB::Expr::ONE - dest_selectors[i].clone()) 
                              + result.clone() * dest_selectors[i].clone();
            when_transition.assert_eq(next_regs[i].clone(), expected_next);
        }
    }
}

/// Generate ALU execution trace
impl AluChip {
    pub fn generate_trace<F: PrimeField64>(
        program: Vec<Instruction>,
        initial_regs: [F; 4],
    ) -> RowMajorMatrix<F> {
        let n = program.len();
        assert!(n > 0, "Program cannot be empty");
        
        // Ensure length is power of 2 (for Plonky3 compatibility)
        let trace_len = if n.is_power_of_two() { n } else { n.next_power_of_two() };
        
        let mut trace = RowMajorMatrix::new(
            F::zero_vec(trace_len * NUM_ALU_COLS),
            NUM_ALU_COLS,
        );

        let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<AluRow<F>>() };
        assert!(prefix.is_empty(), "Alignment should match");
        assert!(suffix.is_empty(), "Alignment should match");
        assert_eq!(rows.len(), trace_len);

        // Initialize current register state
        let mut current_regs = initial_regs;

        // Process each instruction
        for (i, instruction) in program.iter().enumerate() {
            // Create current row (18 columns)
            let mut row = AluRow {
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
                op_add: F::from_u64(0),
                op_sub: F::from_u64(0),
            };

            // Set destination register selector (one-hot encoding)
            match instruction.dest {
                0 => row.dest_0 = F::from_u64(1),
                1 => row.dest_1 = F::from_u64(1),
                2 => row.dest_2 = F::from_u64(1),
                3 => row.dest_3 = F::from_u64(1),
                _ => panic!("Invalid dest register: {}", instruction.dest),
            }

            // Set source1 register selector (one-hot encoding)
            match instruction.src1 {
                0 => row.src1_0 = F::from_u64(1),
                1 => row.src1_1 = F::from_u64(1),
                2 => row.src1_2 = F::from_u64(1),
                3 => row.src1_3 = F::from_u64(1),
                _ => panic!("Invalid src1 register: {}", instruction.src1),
            }

            // Set source2 register selector (one-hot encoding)
            match instruction.src2 {
                0 => row.src2_0 = F::from_u64(1),
                1 => row.src2_1 = F::from_u64(1),
                2 => row.src2_2 = F::from_u64(1),
                3 => row.src2_3 = F::from_u64(1),
                _ => panic!("Invalid src2 register: {}", instruction.src2),
            }

            // Set operation selector (one-hot encoding)
            match instruction.op {
                Opcode::ADD => {
                    row.op_add = F::from_u64(1);
                    row.op_sub = F::from_u64(0);
                }
                Opcode::SUB => {
                    row.op_add = F::from_u64(0);
                    row.op_sub = F::from_u64(1);
                }
            }

            // Add row to trace
            rows[i] = row;

            // Update register state (prepare for next row)
            let src1_val = current_regs[instruction.src1];
            let src2_val = current_regs[instruction.src2];
            
            let result = match instruction.op {
                Opcode::ADD => src1_val + src2_val,
                Opcode::SUB => src1_val - src2_val,
            };
            
            current_regs[instruction.dest] = result;
        }

        // If padding additional rows is needed (maintain last state)
        for i in n..trace_len {
            rows[i] = rows[n - 1];
        }

        trace
    }
}

// Type definitions (same as previous examples)
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
    println!("🔧 Starting Universal Arithmetic Logic Unit (ALU) zero-knowledge prover...");

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

    // Create ALU Chip
    let chip = AluChip;

    // Define test program: mixed ADD and SUB instructions
    println!("📝 Defining ALU test program:");
    let mut program = vec![
        Instruction { op: Opcode::ADD, dest: 0, src1: 0, src2: 1 }, // r0 = r0 + r1 = 1 + 2 = 3
        Instruction { op: Opcode::SUB, dest: 1, src1: 2, src2: 0 }, // r1 = r2 - r0 = 5 - 3 = 2
        Instruction { op: Opcode::ADD, dest: 3, src1: 0, src2: 1 }, // r3 = r0 + r1 = 3 + 2 = 5
        Instruction { op: Opcode::SUB, dest: 2, src1: 3, src2: 1 }, // r2 = r3 - r1 = 5 - 2 = 3
    ];

    // Add more instructions to reach 2^6 = 64 instructions
    while program.len() < 64 {
        program.push(Instruction { op: Opcode::ADD, dest: 0, src1: 0, src2: 1 }); // r0 = r0 + r1
        if program.len() < 64 {
            program.push(Instruction { op: Opcode::SUB, dest: 1, src1: 1, src2: 0 }); // r1 = r1 - r0
        }
        if program.len() < 64 {
            program.push(Instruction { op: Opcode::ADD, dest: 2, src1: 2, src2: 3 }); // r2 = r2 + r3
        }
        if program.len() < 64 {
            program.push(Instruction { op: Opcode::SUB, dest: 3, src1: 3, src2: 2 }); // r3 = r3 - r2
        }
    }

    // Display only first few important instructions
    for (i, inst) in program.iter().take(8).enumerate() {
        let op_str = match inst.op {
            Opcode::ADD => "ADD",
            Opcode::SUB => "SUB",
        };
        println!("  Instruction {}: {} r{} = r{} {} r{}", 
                 i + 1, op_str, inst.dest, inst.src1, 
                 if inst.op == Opcode::ADD { "+" } else { "-" }, 
                 inst.src2);
    }
    if program.len() > 8 {
        println!("  ... and {} more instructions", program.len() - 8);
    }

    // Set initial register state
    let initial_regs = [
        Val::from_u64(1), // r0 = 1
        Val::from_u64(2), // r1 = 2  
        Val::from_u64(5), // r2 = 5
        Val::from_u64(0)  // r3 = 0
    ];
    println!("🏁 Initial register state: [r0={}, r1={}, r2={}, r3={}]", 
             initial_regs[0].as_canonical_u64(),
             initial_regs[1].as_canonical_u64(), 
             initial_regs[2].as_canonical_u64(),
             initial_regs[3].as_canonical_u64());

    // Generate execution trace
    println!("📊 Generating execution trace...");
    let trace = AluChip::generate_trace::<Val>(program.clone(), initial_regs);

    // Manual verification of execution results (display only first few steps and final result)
    println!("✨ ALU execution process:");
    let mut regs = [1u64, 2u64, 5u64, 0u64];
    println!("  Initial state: [r0={}, r1={}, r2={}, r3={}]", regs[0], regs[1], regs[2], regs[3]);
    
    for (i, inst) in program.iter().enumerate() {
        let src1_val = regs[inst.src1];
        let src2_val = regs[inst.src2];
        
        let result = match inst.op {
            Opcode::ADD => src1_val + src2_val,
            Opcode::SUB => {
                if src1_val >= src2_val {
                    src1_val - src2_val
                } else {
                    // Subtraction in finite field
                    let p = (1u64 << 31) - (1u64 << 27) + 1; // BabyBear modulus
                    (src1_val + p - src2_val) % p
                }
            }
        };
        
        regs[inst.dest] = result;
        
        // Display detailed execution process for only first 6 steps
        if i < 6 {
            let op_str = match inst.op {
                Opcode::ADD => "+",
                Opcode::SUB => "-",
            };
            println!("  Execute instruction {}: r{} = r{} {} r{} = {} {} {} = {} -> [r0={}, r1={}, r2={}, r3={}]", 
                     i + 1, inst.dest, inst.src1, op_str, inst.src2, 
                     src1_val, op_str, src2_val, result,
                     regs[0], regs[1], regs[2], regs[3]);
        }
    }
    
    if program.len() > 6 {
        println!("  ... executed {} more instructions", program.len() - 6);
    }

    println!("🎯 Final register state: [r0={}, r1={}, r2={}, r3={}]", regs[0], regs[1], regs[2], regs[3]);

    // Generate proof
    println!("🔐 Generating STARK proof...");
    let proof = prove(&config, &chip, trace, &vec![]);
    println!("✅ Proof generation completed!");

    // Verify proof
    println!("🔍 Verifying proof...");
    match verify(&config, &chip, &proof, &vec![]) {
        Ok(_) => println!("🎉 Proof verification successful! ALU execution correctness has been proven."),
        Err(e) => println!("❌ Proof verification failed: {:?}", e),
    }

    println!("🏁 Universal Arithmetic Logic Unit (ALU) zero-knowledge prover completed!");
    
    // 🤔 Reflection Questions
    println!("\n💭 Reflection Questions:");
    println!("1. Power of conditional constraints: How do operation selectors implement if/else logic?");
    println!("2. If using single is_sub field instead of op_add/op_sub, how should constraints be modified?");
    println!("3. How to further add MUL instruction? Which parts need modification?");
} 
