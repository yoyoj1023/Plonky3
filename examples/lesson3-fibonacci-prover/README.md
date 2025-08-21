This is a Plonky3 implementation challenge based on the official Fibonacci example from the Plonky3 website, designed to guide learners in building a working project from scratch.

---

### **Plonky3 Implementation Challenge: Building a Fibonacci Prover**

#### **Objective**

This exercise is designed to guide you in using Rust and the Plonky3 library to implement a zero-knowledge proof system for proving Fibonacci sequence calculations. Upon completion, you will be able to:

1.  **Define a Plonky3 "Chip"**: Understand how to encapsulate specific computations (Fibonacci sequence) into reusable Plonky3 components.
2.  **Generate Execution Traces**: Produce valid traces for Fibonacci sequence computational processes.
3.  **Implement Algebraic Intermediate Representation (AIR)**: Use Plonky3's `Machine` trait to define initial constraints and transition constraints for computations.
4.  **Generate and Verify Proofs**: Write main programs to actually generate STARK proofs and verify their validity.

#### **Prerequisites**

1.  **Rust Environment**: Ensure you have installed the latest stable version of Rust and Cargo.
    *   You can check with `rustup --version` and `cargo --version`.
2.  **Git**: You need Git to clone the Plonky3 official repository.
3.  **Clone Plonky3 Repository**: This project requires dependencies on Plonky3 core components. Clone the official repository to your local machine:
    ```bash
    git clone https://github.com/Plonky3/Plonky3.git
    cd Plonky3
    ```
    *(The following steps assume you are operating in the `Plonky3` directory)*

#### **Problem Description**

The Fibonacci sequence is a classic recursive sequence defined as:
*   `F(0) = 0`
*   `F(1) = 1`
*   `F(n) = F(n-1) + F(n-2)` for `n > 1`

We will build a proof claiming "we correctly computed N iterations of the Fibonacci sequence". To achieve this, our execution trace will contain two columns: `a` and `b`. In row `i`, these two columns will correspond to `F(i)` and `F(i+1)` respectively.

Therefore, the correctness of the entire computation can be guaranteed by the following rules:
1.  **Initial State**: In the first row (row 0), `a` must be 0, and `b` must be 1.
2.  **State Transition**: For any row `i`, the values in the next row `i+1` must satisfy:
    *   `next_row.a` should equal `current_row.b`
    *   `next_row.b` should equal `current_row.a + current_row.b`

Your task is to translate these rules into code using Plonky3's API.

#### **Implementation Steps**

**Step 1: Project Setup**

1.  In the `Plonky3/examples` directory, create a new rust file, for example `my_fibonacci.rs`.
2.  You can refer to the structure of `Plonky3/examples/fibonacci.rs`, but we strongly recommend typing out the code yourself rather than copying and pasting to deepen understanding.

**Step 2: Define `FibonacciChip`**

1.  Create a public structure `pub struct FibonacciChip;`.
2.  Implement the `Chip` trait for it. This Chip doesn't need any configuration or fields, so you can specify `Config` and `Bus` as `()` and `()`.

**Step 3: Implement `Machine` Trait to Define AIR**

This is the core of this exercise. You need to implement the `Machine` trait for `FibonacciChip`.

1.  **`eval()` Function**: This function is where all algebraic constraints are defined. It receives a `builder` object that you can use to add constraints.
    *   **Access Trace Fields**: You need to first get the current row (local) and next row (next) fields from `builder`'s `main.local_mut()` and `main.next_mut()`. Our trace has two columns, which can be accessed like this:
        ```rust
        let local_slice = main.local_mut();
        let a = local_slice[0];
        let b = local_slice[1];

        let next_slice = main.next_mut();
        let next_a = next_slice[0];
        let next_b = next_slice[1];
        ```
    *   **Define Initial Constraints**: Use `builder.when_first_row()` to define constraints that only apply to the first row. You need to ensure `a` equals 0 and `b` equals 1.
        ```rust
        // builder.when_first_row().assert_eq(a, F::zero());
        // ...
        ```
    *   **Define Transition Constraints**: Use `builder.when_transition()` to define constraints that apply to all state transitions. You need to state `next_a == b` and `next_b == a + b` here.
        ```rust
        // builder.when_transition().assert_eq(next_a, b);
        // ...
        ```

**Step 4: Generate Execution Trace**

1.  Implement a helper function for `FibonacciChip`, for example `generate_trace`, which takes `n` (number of iterations) as input and returns a `RowMajorMatrix<F>`.
2.  The logic of this function should be:
    *   Initialize an empty trace vector.
    *   Set initial values `a = F::zero()`, `b = F::one()`.
    *   Loop `n` times, each time:
        *   Add the row `[a, b]` to the trace.
        *   Update the values of `a` and `b` to conform to Fibonacci sequence rules (`let next_b = a + b; a = b; b = next_b;`).
    *   Convert the generated vector to `RowMajorMatrix`.

**Step 5: Write `main` Function for Proof and Verification**

1.  In the `main` function, set up your Runtime and Chip.
2.  Call the `generate_trace` function you created in step 4 to generate a trace with 1000 rows.
3.  Use the `chip.prove()` function, passing Runtime, Config, and Trace to generate the proof.
4.  Use the `chip.verify()` function, passing Runtime, Config, and proof to verify the proof's validity.
5.  Print prompt messages before and after proof generation and verification, and print success messages after successful verification.

#### **Verification and Reflection**

1.  **Execution**: In the `Plonky3` root directory, use the following command to execute your program:
    ```bash
    cargo run --release --example my_fibonacci
    ```
    If everything goes smoothly, you should see messages indicating successful proof generation and verification.

2.  **Reflection Questions (please answer in your submission documentation)**:
    *   **Breaking the Trace**: If after generating the trace but before `prove()`, you manually modify a value in the trace (e.g., `trace.values[10] = F::random(rng)`), what happens when you run it again? Why?
    *   **Breaking Constraints**: If in the `eval()` function, you incorrectly write the transition constraint `next_b == a + b` as `next_b == a + b + F::one()`, but use the original correct trace, what happens when you execute `prove()`? Why?
    *   **Generality**: How would you modify your code to prove the Lucas sequence, which follows the rule `L(n) = L(n-1) + L(n-2)` but with initial values `L(0) = 2`, `L(1) = 1`?

#### **Submission Requirements**

1.  A GitHub repository link containing all your code.
2.  A `README.md` file including:
    *   Simple instructions on how to run your project.
    *   Detailed answers to the three questions in the "Verification and Reflection" section above.