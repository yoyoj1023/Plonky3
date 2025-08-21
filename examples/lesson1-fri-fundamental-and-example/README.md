### Exercise:

Suppose we work in the finite field F₁₇ (i.e., all operations are performed modulo 17).

We have a polynomial f(x) = x³ + 2x² + 3x + 4.

Our evaluation domain is a symmetric point set S₀ = {-4, -3, -2, -1, 1, 2, 3, 4}.

**Tasks:**

1.  Calculate all values of polynomial f(x) on domain S₀.
2.  Perform one FRI folding on these values to generate a new polynomial f₁(x) and a new domain S₁.
3.  Actually compute and verify the folding results.

---

### Step 1: Calculate Values of Original Polynomial on Domain

First, we calculate f(x) = x³ + 2x² + 3x + 4 on S₀ = {-4, -3, -2, -1, 1, 2, 3, 4}. All calculations are performed in F₁₇.

*   f(1) = 1³ + 2(1)² + 3(1) + 4 = 1 + 2 + 3 + 4 = 10 (mod 17)
*   f(-1) = (-1)³ + 2(-1)² + 3(-1) + 4 = -1 + 2 - 3 + 4 = 2 (mod 17)
*   f(2) = 2³ + 2(2)² + 3(2) + 4 = 8 + 8 + 6 + 4 = 26 ≡ 9 (mod 17)
*   f(-2) = (-2)³ + 2(-2)² + 3(-2) + 4 = -8 + 8 - 6 + 4 = -2 ≡ 15 (mod 17)
*   f(3) = 3³ + 2(3)² + 3(3) + 4 = 27 + 18 + 9 + 4 = 58 ≡ 7 (mod 17)
*   f(-3) = (-3)³ + 2(-3)² + 3(-3) + 4 = -27 + 18 - 9 + 4 = -14 ≡ 3 (mod 17)
*   f(4) = 4³ + 2(4)² + 3(4) + 4 = 64 + 32 + 12 + 4 = 112 ≡ 10 (mod 17)
*   f(-4) = (-4)³ + 2(-4)² + 3(-4) + 4 = -64 + 32 - 12 + 4 = -40 ≡ 11 (mod 17)

So, the commitment we get is this list of values:

C₀ = {f(1), f(-1), f(2), f(-2), f(3), f(-3), f(4), f(-4)}

C₀ = {10, 2, 9, 15, 7, 3, 10, 11}

### Step 2: Perform FRI Folding

The core idea of FRI is to use symmetry to fold polynomials. We decompose polynomial f(x) into even part g(x²) and odd part x · h(x²).

f(x) = (2x² + 4) + x(x² + 3)

Here:
*   The even part corresponds to function g(y) = 2y + 4
*   The odd part corresponds to function h(y) = y + 3

The Prover will choose a random challenge value α (provided by the Verifier). We assume the Verifier chose α = 5.

Now, the Prover constructs a new polynomial f₁(x):
f₁(y) = g(y) + α · h(y) = (2y + 4) + 5(y + 3) = 2y + 4 + 5y + 15 = 7y + 19 ≡ 7y + 2 (mod 17)

So, the new, lower-degree polynomial is f₁(y) = 7y + 2.

At the same time, we also need a new domain S₁. This new domain is formed by pairing symmetric points in S₀ and taking their squares:
S₁ = {1², 2², 3², 4²} = {1, 4, 9, 16}
Since 16 ≡ -1 (mod 17), we can also write S₁ = {1, 4, 9, -1}.

#### Second Folding: From Linear to Constant Polynomial

Now we need to perform a second folding on the linear polynomial f₁(y) = 7y + 2.

First, calculate f₁(y) = 7y + 2 on domain S₁ = {1, 4, 9, 16}:

*   f₁(1) = 7(1) + 2 = 9
*   f₁(4) = 7(4) + 2 = 28 + 2 = 30 ≡ 13 (mod 17)
*   f₁(9) = 7(9) + 2 = 63 + 2 = 65 ≡ 14 (mod 17) (since 65 = 3 × 17 + 14)
*   f₁(16) = f₁(-1) = 7(-1) + 2 = -7 + 2 = -5 ≡ 12 (mod 17)

So the second round commitment is:
C₁ = {f₁(1), f₁(4), f₁(9), f₁(-1)} = {9, 13, 14, 12}

For the linear polynomial f₁(y) = 7y + 2, we can decompose it as:
*   Even part (constant term): g₁(z) = 2
*   Odd part coefficient: h₁(z) = 7

The Verifier provides a second challenge value, assume α₂ = 3.

The result of the second folding is a constant polynomial:
f₂(z) = g₁(z) + α₂ · h₁(z) = 2 + 3 × 7 = 2 + 21 = 23 ≡ 6 (mod 17)

So, after two foldings, we get constant polynomial f₂(z) = 6.

The new domain S₂ corresponds to pairing symmetric points in S₁. In this example, we can pair the points as:
*   Pair 1: (1, -1) → representative element is 1² = 1
*   Pair 2: (4, 9) → since 2² = 4 and 3² = 9, we can choose representative element 4

So S₂ = {1, 4}.

The final constant polynomial f₂(z) = 6 has value 6 at any point:
*   f₂(1) = 6
*   f₂(4) = 6

The third round commitment is:
C₂ = {6, 6}

#### Final Folding: Reaching Constant

Since we have reached a constant polynomial, the FRI process can end. The Verifier can directly verify this constant value without further queries.

### Step 3: Actual Query and Verification

Now, the Verifier needs to verify whether the Prover's computation is correct. The Verifier will not compute the entire C₁, but will randomly sample a point.

Assume the Verifier randomly chose point x₀ = 2 for query.

1.  **Verifier's Request:**
    The Verifier will ask the Prover for three values: f(2), f(-2) and f₁(2²), which is f₁(4).

2.  **Prover's Response:**
    *   f(2) = 9
    *   f(-2) = 15
    *   f₁(4) = 7(4) + 2 = 28 + 2 = 30 ≡ 13 (mod 17)

3.  **Verifier's Verification Calculation:**
    The Verifier now needs to verify whether these three values satisfy the folding relation. The core relation of FRI folding is:
    
    ```
    f₁(x₀²) = [f(x₀) + f(-x₀)]/2 + α · [f(x₀) - f(-x₀)]/(2x₀)
    ```

    We substitute the values provided by the Prover into the right side of the formula:
    
    *   **First part (even part contribution):**
        [f(2) + f(-2)]/2 = (9 + 15)/2 = 24/2 = 12
        Here we need to compute 2⁻¹ (mod 17). Since 9 × 2 = 18 ≡ 1 (mod 17), so 2⁻¹ ≡ 9 (mod 17).
        So, 24/2 = 24 × 9 = 216 ≡ (12 × 17 + 12) ≡ 12 (mod 17).

    *   **Second part (odd part contribution):**
        α · [f(2) - f(-2)]/(2x₀) = 5 · (9 - 15)/(2(2)) = 5 · (-6)/4 = 5 · 11/4
        We need to compute 4⁻¹ (mod 17). Since 13 × 4 = 52 ≡ 1 (mod 17), so 4⁻¹ ≡ 13 (mod 17).
        So, 11/4 = 11 × 13 = 143 ≡ (8 × 17 + 7) ≡ 7 (mod 17).
        The entire second part equals 5 · 7 = 35 ≡ 1 (mod 17).

    *   **Combined result:**
        12 + 1 = 13 (mod 17)

4.  **Comparison:**
    The Verifier's calculated result is **13**.
    The Prover's provided f₁(4) value is also **13**.

    They match perfectly! This shows that, at least at point x₀=2, the Prover's first folding calculation is honest.

### Verifying Second Folding: From Linear to Constant Polynomial

Now we need to verify the correctness of the second folding, i.e., the process from f₁(y) = 7y + 2 folding to constant polynomial f₂(z) = 6.

Assume the Verifier randomly chose y₀ = 4 for the second query.

1.  **Verifier's Request:**
    The Verifier will ask the Prover for the following values:
    - f₁(4) = 13
    - f₁(-4) = f₁(13) (since -4 ≡ 13 (mod 17))
    - f₂(4²) = f₂(16) = f₂(-1) = 6 (constant polynomial has value 6 at any point)

2.  **Calculate f₁(-4) = f₁(13):**
    f₁(13) = 7(13) + 2 = 91 + 2 = 93 ≡ 8 (mod 17) (since 93 = 5 × 17 + 8)

3.  **Prover's Response:**
    *   f₁(4) = 13
    *   f₁(-4) = f₁(13) = 8
    *   f₂(16) = 6

4.  **Verifier's Verification Calculation:**
    For the second folding, we have f₁(y) = 7y + 2, decomposed as:
    - Even part (constant term): g₁(z) = 2
    - Odd part coefficient: h₁(z) = 7
    
    The verification relation is:
    ```
    f₂(y₀²) = g₁(y₀²) + α₂ · h₁(y₀²)
    ```
    
    But for linear polynomials, we can use a more direct formula:
    ```
    f₂(y₀²) = [f₁(y₀) + f₁(-y₀)]/2 + α₂ · [f₁(y₀) - f₁(-y₀)]/(2y₀)
    ```

    Substituting values:
    *   **First part (even part contribution):**
        [f₁(4) + f₁(-4)]/2 = (13 + 8)/2 = 21/2
        Calculate 21/2 = 21 × 9 = 189 ≡ (11 × 17 + 2) ≡ 2 (mod 17)

    *   **Second part (odd part contribution):**
        α₂ · [f₁(4) - f₁(-4)]/(2y₀) = 3 · (13 - 8)/(2(4)) = 3 · 5/8
        We need to compute 8⁻¹ (mod 17). Since 15 × 8 = 120 ≡ 1 (mod 17), so 8⁻¹ ≡ 15 (mod 17).
        So, 5/8 = 5 × 15 = 75 ≡ (4 × 17 + 7) ≡ 7 (mod 17).
        The entire second part equals 3 × 7 = 21 ≡ 4 (mod 17).

    *   **Combined result:**
        2 + 4 = 6 (mod 17)

5.  **Comparison:**
    The Verifier's calculated result is **6**.
    The Prover's provided f₂(16) value is also **6**.

    They match perfectly! This shows the second folding is also honest.

### Final Verification

Since we have reached constant polynomial f₂(z) = 6, the Verifier can directly check this constant value. A constant polynomial should have the same value at all points, which is very easy to verify.

**Complete FRI Verification Summary:**
1. ✅ First folding verification passed: f(x) → f₁(y)
2. ✅ Second folding verification passed: f₁(y) → f₂(z) = 6
3. ✅ Final constant polynomial verification: f₂(z) = 6 is a valid constant

Through this complete FRI process, the Verifier can be highly confident that the Prover's provided original commitment C₀ indeed represents the evaluations of a low-degree polynomial (degree less than 8) without checking all points.

---

## Supplement: Complete Derivation of FRI Folding Core Relation

### Step 1: Even-Odd Decomposition of Polynomials

Any polynomial P(x) can be uniquely decomposed into even and odd parts:

P(x) = P_even(x) + P_odd(x)

Where:
- **Even part** P_even(x): contains only even-degree terms, can be written as G(x²)
- **Odd part** P_odd(x): contains only odd-degree terms, can be written as x · H(x²)

Therefore:
P(x) = G(x²) + x · H(x²)

**Why this decomposition?**
- Even-degree terms have the same values at x and -x
- Odd-degree terms have opposite values at x and -x
- This symmetry is the core of FRI folding

### Step 2: Establish System of Equations

Substituting -x into the same formula:

P(-x) = G((-x)²) + (-x) · H((-x)²)

P(-x) = G(x²) - x · H(x²)

Now we have a system of two linear equations in G(x²) and H(x²):

```
P(x) = G(x²) + x · H(x²)    ...(1)
P(-x) = G(x²) - x · H(x²)   ...(2)
```

### Step 3: Solve for Even and Odd Parts

Through simple addition and subtraction, we can **uniquely solve** for G(x²) and H(x²):

**Solve for even part:** (1) + (2)

P(x) + P(-x) = 2 · G(x²) 

⇒ G(x²) = [P(x) + P(-x)]/2

**Solve for odd part:** (1) - (2)

P(x) - P(-x) = 2x · H(x²) 

⇒ H(x²) = [P(x) - P(-x)]/(2x)

### Step 4: Introduce Random Challenge Value

In the FRI protocol, the Verifier provides a random challenge value α. The Prover uses this challenge value to construct a new polynomial:

P_next(x²) = G(x²) + α · H(x²)

**Why do we need challenge value α?**
1. **Prevent cheating**: Without α, the Prover might provide fake G and H
2. **Randomness**: The randomness of α ensures the Prover cannot prepare fake proofs in advance
3. **Uniqueness**: Each random α produces a different linear combination

### Step 5: Derive FRI Folding Core Relation

Substituting the previous results:

**P_next(x²) = [P(x) + P(-x)]/2 + α · [P(x) - P(-x)]/(2x)**

This is the **FRI folding core relation**!

### Step 6: Mathematical Meaning of Verification Process

**Verifier's Logic:**
1. If the Prover is honest, then the provided P(x), P(-x) and P_next(x²) should satisfy the above relation
2. Through random sampling, the Verifier can detect dishonest Provers with high probability
3. After multiple rounds of folding, we eventually get a constant polynomial, which is very easy to verify

**Key Insights:**
- Each folding halves the polynomial degree
- Domain size also halves
- But verification security does not decrease (due to random challenge values)

### Example Verification

Using our example, first folding:
- P(x) = x³ + 2x² + 3x + 4
- At point x = 2: P(2) = 9, P(-2) = 15
- Challenge value: α = 5

Calculate:
P_next(4) = (9 + 15)/2 + 5 · (9 - 15)/(2 × 2) = 12 + 1 = 13

This exactly matches our direct calculation f₁(4) = 7 × 4 + 2 = 30 ≡ 13 (mod 17)!