Following the Fibonacci example, building a "universal adder" capable of executing instructions is an excellent advanced exercise. This problem guides learners to think about how to handle "state" and "configurable operations", which are the foundation for building more complex ZK-VMs (zero-knowledge virtual machines).

This is an implementation exercise based on this idea.

---

### **Plonky3 Implementation Challenge: Building a Universal Adder CPU**

#### **Objective**

In the previous exercise, we built proofs for a fixed computation (Fibonacci sequence). Now, we will challenge a more general scenario: a simple CPU that can execute a series of "addition instructions".

This exercise is designed to guide you in implementing a system that can prove "given an initial state and a series of instructions, we correctly executed all addition operations and obtained the final state". Upon completion, you will learn:

1.  **State Management**: How to represent and update a set of registers in the execution trace.
2.  **Using Selectors**: How to use binary selector fields to dynamically decide which operation to execute in each row.
3.  **Defining More Complex AIR**: Writing algebraic constraints for computations with conditional logic.
4.  **Generating Traces from "Programs"**: Converting an abstract instruction sequence into concrete execution traces required by Plonky3.

#### **Concept: A CPU with Only `ADD` Instructions**

Imagine a minimalist CPU with 4 registers (`r0`, `r1`, `r2`, `r3`). It can only understand one type of instruction:

`ADD dest, src1, src2`

This instruction means "add the values of `src1` register and `src2` register, and store the result in `dest` register". For example, `ADD r0, r1, r2` would execute `r0 = r1 + r2`. Other registers (in this example, `r3`) remain unchanged.

Our goal is to prove that a "program" (a series of `ADD` instructions) was executed correctly.

#### **Trace Design**

To prove this, our execution trace needs to capture two things in each row:
1.  **Current State**: The values of all registers before executing the instruction.
2.  **Current Instruction**: Which `ADD` operation we want to execute.

Therefore, our trace will contain the following fields (using 4 registers as an example):

| Field Name | Description |
| :--- | :--- |
| `r0`, `r1`, `r2`, `r3` | **Value Fields**: Values of the 4 registers *before* executing this row's instruction. |
| `dest_0`, `dest_1`, `dest_2`, `dest_3` | **Destination Selectors**: One-hot encoding. If `dest_i` is 1, it means `ri` is the destination register. |
| `src1_0`, `src1_1`, `src1_2`, `src1_3` | **Source1 Selectors**: One-hot encoding. If `src1_i` is 1, it means `ri` is the source1 register. |
| `src2_0`, `src2_1`, `src2_2`, `src2_3` | **Source2 Selectors**: One-hot encoding. If `src2_i` is 1, it means `ri` is the source2 register. |

Total: `4 + 4*3 = 16` fields.

**State Transition Rules**:
From row `i` to row `i+1`, register values must be updated according to the selectors in row `i`.
*   **For destination register `dest`**: `next_row.dest = current_row.src1 + current_row.src2`.
*   **For non-destination registers**: `next_row.reg = current_row.reg` (value remains unchanged).

#### **Implementation Steps**

**Step 1: Project Setup and Data Structures**

1.  Create `my_adder.rs` in the `Plonky3/examples` directory.
2.  To conveniently represent instructions, first define a simple structure:
    ```rust
    #[derive(Clone, Copy)]
    struct Instruction {
        dest: usize, // register index 0-3
        src1: usize,
        src2: usize,
    }
    ```
3.  Define the `AdderChip` structure `pub struct AdderChip;` and implement the `Chip` trait for it.

**Step 2: Generate Execution Trace**

This is the key to converting abstract instructions into concrete traces. Implement a helper function for `AdderChip`:
`generate_trace(&self, program: Vec<Instruction>, initial_regs: [F; 4]) -> RowMajorMatrix<F>`

1.  **Initialization**: Set up a `current_regs` variable to track current register values, initialized to `initial_regs`. Initialize an empty `trace` vector.
2.  **Loop Through Instructions**: Iterate through each `Instruction` in `program`.
    *   **Create Row**: Create a 16-column `row` vector for the current instruction.
    *   **Fill Values**: Fill the first 4 positions of `row` with `current_regs` values.
    *   **Fill Selectors**: Based on the current `Instruction`'s `dest`, `src1`, `src2` indices, set the corresponding one-hot selector fields to `F::one()`, others to `F::zero()`.
    *   **Add to Trace**: Add the completed `row` to the `trace` vector.
    *   **Update State**: Calculate the *next* state's register values based on the instruction and update the `current_regs` variable for the next loop iteration.
3.  **Return**: Convert the `trace` vector to `RowMajorMatrix` and return.

**Step 3: Implement `Machine` Trait to Define AIR**

Implement the `Machine` trait for `AdderChip` and define constraints in the `eval()` function.

1.  **Get Fields**: Get all 16 fields from current and next rows from `main.local()` and `main.next()`. It would be helpful to group them into `local_regs`, `local_selectors`, and `next_regs`.
2.  **Constraint 1: Selector Validity**
    *   Use `builder.assert_bool(s)` to ensure all 12 selector fields have values of 0 or 1.
    *   Use `builder.assert_one(...)` to ensure each group of selectors (dest, src1, src2) is one-hot. For example: `builder.assert_one(local_selectors.dest_0 + local_selectors.dest_1 + ...)`.
3.  **Constraint 2: State Transition**
    *   **Calculate Source Values**: Use selectors and register values to calculate `src1_val` and `src2_val`. This can be elegantly implemented through dot product:
        `src1_val = local_regs[0] * local_selectors.src1_0 + local_regs[1] * local_selectors.src1_1 + ...`
    *   **Calculate Addition Result**: `add_result = src1_val + src2_val`.
    *   **Constrain Each Register's Next State**: For each register `i` (from 0 to 3), create the following constraint:
        `next_regs[i] = local_regs[i] * (F::one() - local_selectors.dest_i) + add_result * local_selectors.dest_i`
        The meaning of this formula is:
        *   If `dest_i` is 1 (this register is the destination), then `1 - dest_i` is 0, next state equals `add_result`.
        *   If `dest_i` is 0 (this register is not the destination), then `1 - dest_i` is 1, next state equals `local_regs[i]` (remains unchanged).
        Use `builder.assert_eq(...)` to impose this constraint.

**Step 4: Write `main` Function for Proof and Verification**

1.  **Define a Program**: Create a `Vec<Instruction>` as your test program. For example:
    ```rust
    // r2 = r0 + r1   (1 + 2 = 3)
    // r3 = r2 + r2   (3 + 3 = 6)
    // r0 = r3 + r1   (6 + 2 = 8)
    let program = vec![
        Instruction { dest: 2, src1: 0, src2: 1 },
        Instruction { dest: 3, src1: 2, src2: 2 },
        Instruction { dest: 0, src1: 3, src2: 1 },
    ];
    ```
2.  **Set Initial State**: Define initial values for the 4 registers, for example `[1, 2, 0, 0]`.
3.  **Generate Trace, Proof & Verification**: Call your `generate_trace` function, then call `prove()` and `verify()`.

#### **Verification and Reflection**

1.  **Execution**: Use `cargo run --release --example my_adder` to execute your program and confirm both proof and verification succeed.
2.  **Manual Verification**: After your program completes, what should the final row's register state be? Calculate manually and compare with the last state in your trace to confirm your `generate_trace` logic is correct.
3.  **Reflection Questions (please answer in your submission documentation)**:
    *   **Invalid Instructions**: If during trace generation, you accidentally set all `dest` selectors in a row to 0, at which step would `prove()` fail? Which constraint would be violated?
    *   **State Tampering**: If after `generate_trace` but before `prove`, you manually modify a non-destination register's value (e.g., in the `r0 = r3 + r1` row, you also manually modify `r2`'s value), at which step would `prove()` fail? Which constraint would be violated?
    *   **Instruction Set Extension**: If you wanted to add a `MOV dest, src` instruction (copy `src` value to `dest`) to this CPU, how would you modify your trace design and constraints in the `eval()` function? (Hint: might need a new `op_selector` field to distinguish between `ADD` and `MOV`).