## 🚀 Plonky3 True/False Quiz 🚀

### 📝 Questions, Answers, and Explanations

1.  **Answer: ( ✓ )** 
    *   **Question:** In Plonky3, the main purpose of **AIR** (Algebraic Intermediate Representation) is to transform a computational problem into a set of algebraic constraints about polynomials.
    *   **Explanation:** Correct. The core purpose of AIR is to transform the statement "this computation is correct" into a set of algebraic identities that polynomials must satisfy.

2.  **Answer: ( ✗ )**
    *   **Question:** The execution **Trace** is a table that records the program's state at each time point, but it has no direct relationship with the constraints defined in **AIR**.
    *   **Explanation:** Incorrect. The execution trace (Trace) and AIR constraints are inseparable. It is precisely these constraints that are applied to each row of the Trace to ensure the correctness of the entire computational process.

3.  **Answer: ( ✗ )**
    *   **Question:** The **Fibonacci sequence** cannot be expressed using Plonky3's **AIR** due to its recursive nature.
    *   **Explanation:** Incorrect. The Fibonacci sequence is a classic and fundamental teaching example in Plonky3. Its recursive relationship `a_n = a_{n-1} + a_{n-2}` can be very intuitively transformed into constraints in AIR.

4.  **Answer: ( ✓ )**
    *   **Question:** The core purpose of the **FRI** (Fast Reed-Solomon Interactive Oracle Proof of Proximity) protocol in Plonky3 is to efficiently prove that a committed polynomial's degree does not exceed a known upper bound.
    *   **Explanation:** Correct. FRI is a low-degree testing protocol. It allows the verifier to efficiently believe that the polynomial provided by the prover is indeed low-degree without reading the entire polynomial.

5.  **Answer: ( ✓ )**
    *   **Question:** In Plonky3's development workflow, defining the **AIR** is the first step, followed by generating the execution trace (**Trace**) based on this definition.
    *   **Explanation:** Correct. In Plonky3's standard workflow, developers need to first use AIR to precisely define the computational rules and constraints, and then generate the corresponding execution trace based on this definition.

6.  **Answer: ( ✗ )**
    *   **Question:** Plonky3 is a completely universal STARK framework that any STARK-based proof system can use directly without any customization.
    *   **Explanation:** Incorrect. Plonky3 provides a "toolkit" for building efficient proof systems, not a plug-and-play universal framework. Different applications typically need to customize their STARK implementation based on their specific logic.

7.  **Answer: ( ✓ )**
    *   **Question:** When proving the **Fibonacci sequence** calculation, we must define its initial state (e.g., the first two values of the sequence) as part of the "boundary constraints".
    *   **Explanation:** Correct. To ensure the computation starts from a correct starting point, we must lock down the initial values of the sequence through boundary constraints, such as `trace[0] = 0` and `trace[1] = 1`.

8.  **Answer: ( ✓ )**
    *   **Question:** The **FRI** protocol consists of two main phases: "commit" and "query". In the commit phase, the prover recursively folds polynomials and commits to their results.
    *   **Explanation:** Correct. The FRI protocol indeed consists of these two main phases. In the commit phase, the prover continuously folds polynomials and sends corresponding Merkle tree roots; in the query phase, the verifier randomly samples some points to verify whether this folding process is honest.

9.  **Answer: ( ✗ )**
    *   **Question:** Plonky3 can only use one specific finite field and hash function, which limits its flexibility in different application scenarios.
    *   **Explanation:** Incorrect. One of Plonky3's major advantages is its modular design. It supports multiple different finite fields (such as BabyBear, Goldilocks) and hash functions (such as Poseidon2, BLAKE3), allowing developers to freely combine them according to security and performance requirements.

10. **Answer: ( ✓ )**
    *   **Question:** The "constraint polynomials" in **AIR** are used to verify the internal state of each row in the execution trace (**Trace**), as well as whether the state transitions between adjacent rows are all correct.
    *   **Explanation:** Correct. The core responsibility of constraint polynomials is to verify the validity of the Trace. This includes checking whether the values within a single row satisfy conditions (for example, a certain value is 0 or 1), and whether the state transitions between adjacent rows conform to predefined computational logic.

This is a Plonky3 Q&A exercise covering the topics you mentioned: AIR, Trace, Fibonacci, and FRI, with additional key concepts such as polynomial commitment schemes (PCS) and STARKs to provide a more comprehensive learning experience.

These questions progress from basic definitions to integrated applications, aiming to help learners consolidate their knowledge.
