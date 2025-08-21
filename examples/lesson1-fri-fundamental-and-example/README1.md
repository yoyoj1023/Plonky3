### Exercise:

Suppose we work in the finite field $\mathbb{F}_{17}$ (i.e., all operations are performed modulo 17).

We have a polynomial $f(x) = x^3 + 2x^2 + 3x + 4$.

Our evaluation domain is a symmetric point set $S_0 = \{-4, -3, -2, -1, 1, 2, 3, 4\}$.

**Tasks:**

1.  Calculate all values of polynomial $f(x)$ on domain $S_0$.
2.  Perform one FRI folding on these values to generate a new polynomial $f_1(x)$ and a new domain $S_1$.
3.  Actually compute and verify the folding results.

---

### Step 1: Calculate Values of Original Polynomial on Domain

First, we calculate $f(x) = x^3 + 2x^2 + 3x + 4$ on $S_0 = \{-4, -3, -2, -1, 1, 2, 3, 4\}$. All calculations are performed in $\mathbb{F}_{17}$.

*   $f(1) = 1^3 + 2(1)^2 + 3(1) + 4 = 1 + 2 + 3 + 4 = 10 \pmod{17}$
*   $f(-1) = (-1)^3 + 2(-1)^2 + 3(-1) + 4 = -1 + 2 - 3 + 4 = 2 \pmod{17}$
*   $f(2) = 2^3 + 2(2)^2 + 3(2) + 4 = 8 + 8 + 6 + 4 = 26 \equiv 9 \pmod{17}$
*   $f(-2) = (-2)^3 + 2(-2)^2 + 3(-2) + 4 = -8 + 8 - 6 + 4 = -2 \equiv 15 \pmod{17}$
*   $f(3) = 3^3 + 2(3)^2 + 3(3) + 4 = 27 + 18 + 9 + 4 = 58 \equiv 7 \pmod{17}$
*   $f(-3) = (-3)^3 + 2(-3)^2 + 3(-3) + 4 = -27 + 18 - 9 + 4 = -14 \equiv 3 \pmod{17}$
*   $f(4) = 4^3 + 2(4)^2 + 3(4) + 4 = 64 + 32 + 12 + 4 = 112 \equiv 10 \pmod{17}$
*   $f(-4) = (-4)^3 + 2(-4)^2 + 3(-4) + 4 = -64 + 32 - 12 + 4 = -40 \equiv 11 \pmod{17}$

So, the commitment we get is this list of values:
$C_0 = \{f(1), f(-1), f(2), f(-2), f(3), f(-3), f(4), f(-4)\}$

$C_0 = \{10, 2, 9, 15, 7, 3, 10, 11\}$

### Step 2: Perform FRI Folding

The core idea of FRI is to use symmetry to fold polynomials. We decompose polynomial $f(x)$ into even part $g(x^2)$ and odd part $x \cdot h(x^2)$.

$f(x) = (2x^2 + 4) + x(x^2 + 3)$

Here:
*   The even part corresponds to function $g(y) = 2y + 4$
*   The odd part corresponds to function $h(y) = y + 3$

The Prover will choose a random challenge value $\alpha$ (provided by the Verifier). We assume the Verifier chose $\alpha = 5$.

Now, the Prover constructs a new polynomial $f_1(x)$:
$f_1(y) = g(y) + \alpha \cdot h(y) = (2y + 4) + 5(y + 3) = 2y + 4 + 5y + 15 = 7y + 19 \equiv 7y + 2 \pmod{17}$

So, the new, lower-degree polynomial is $f_1(y) = 7y + 2$.

At the same time, we also need a new domain $S_1$. This new domain is formed by pairing symmetric points in $S_0$ and taking their squares:
$S_1 = \{1^2, 2^2, 3^2, 4^2\} = \{1, 4, 9, 16\}$
Since $16 \equiv -1 \pmod{17}$, we can also write $S_1 = \{1, 4, 9, -1\}$.

#### Second Folding: From Linear to Constant Polynomial

Now we need to perform a second folding on the linear polynomial $f_1(y) = 7y + 2$.

First, calculate $f_1(y) = 7y + 2$ on domain $S_1 = \{1, 4, 9, 16\}$:

*   $f_1(1) = 7(1) + 2 = 9$
*   $f_1(4) = 7(4) + 2 = 28 + 2 = 30 \equiv 13 \pmod{17}$
*   $f_1(9) = 7(9) + 2 = 63 + 2 = 65 \equiv 14 \pmod{17}$ (since $65 = 3 \times 17 + 14$)
*   $f_1(16) = f_1(-1) = 7(-1) + 2 = -7 + 2 = -5 \equiv 12 \pmod{17}$

So the second round commitment is:
$C_1 = \{f_1(1), f_1(4), f_1(9), f_1(-1)\} = \{9, 13, 14, 12\}$

For the linear polynomial $f_1(y) = 7y + 2$, we can decompose it as:
*   Even part (constant term): $g_1(z) = 2$
*   Odd part coefficient: $h_1(z) = 7$

The Verifier provides a second challenge value, assume $\alpha_2 = 3$.

The result of the second folding is a constant polynomial:
$f_2(z) = g_1(z) + \alpha_2 \cdot h_1(z) = 2 + 3 \times 7 = 2 + 21 = 23 \equiv 6 \pmod{17}$

So, after two foldings, we get constant polynomial $f_2(z) = 6$.

The new domain $S_2$ corresponds to pairing symmetric points in $S_1$. In this example, we can pair the points as:
*   Pair 1: $(1, -1)$ → representative element is $1^2 = 1$
*   Pair 2: $(4, 9)$ → since $2^2 = 4$ and $3^2 = 9$, we can choose representative element 4

So $S_2 = \{1, 4\}$.

The final constant polynomial $f_2(z) = 6$ has value 6 at any point:
*   $f_2(1) = 6$
*   $f_2(4) = 6$

The third round commitment is:
$C_2 = \{6, 6\}$

#### Final Folding: Reaching Constant

Since we have reached a constant polynomial, the FRI process can end. The Verifier can directly verify this constant value without further queries.

### Step 3: Actual Query and Verification

Now, the Verifier needs to verify whether the Prover's computation is correct. The Verifier will not compute the entire $C_1$, but will randomly sample a point.

Assume the Verifier randomly chose $x_0 = 2$ for query.

1.  **Verifier's Request:**
    The Verifier will ask the Prover for three values: $f(2)$, $f(-2)$ and $f_1(2^2)$, which is $f_1(4)$.

2.  **Prover's Response:**
    *   $f(2) = 9$
    *   $f(-2) = 15$
    *   $f_1(4) = 7(4) + 2 = 28 + 2 = 30 \equiv 13 \pmod{17}$

3.  **Verifier's Verification Calculation:**
    The Verifier now needs to verify whether these three values satisfy the folding relation. The core relation of FRI folding is:
    $$f_1(x_0^2) = \frac{f(x_0) + f(-x_0)}{2} + \alpha \cdot \frac{f(x_0) - f(-x_0)}{2x_0}$$

    We substitute the values provided by the Prover into the right side of the formula:
    *   **First part (even part contribution):**
        $\frac{f(2) + f(-2)}{2} = \frac{9 + 15}{2} = \frac{24}{2} = 12$
        Here we need to compute $2^{-1} \pmod{17}$. Since $9 \times 2 = 18 \equiv 1 \pmod{17}$, so $2^{-1} \equiv 9 \pmod{17}$.
        So, $\frac{24}{2} = 24 \times 9 = 216 \equiv (12 \times 17 + 12) \equiv 12 \pmod{17}$.

    *   **Second part (odd part contribution):**
        $\alpha \cdot \frac{f(2) - f(-2)}{2x_0} = 5 \cdot \frac{9 - 15}{2(2)} = 5 \cdot \frac{-6}{4} = 5 \cdot \frac{11}{4}$
        We need to compute $4^{-1} \pmod{17}$. Since $13 \times 4 = 52 \equiv 1 \pmod{17}$, so $4^{-1} \equiv 13 \pmod{17}$.
        So, $\frac{11}{4} = 11 \times 13 = 143 \equiv (8 \times 17 + 7) \equiv 7 \pmod{17}$.
        The entire second part equals $5 \cdot 7 = 35 \equiv 1 \pmod{17}$.

    *   **Combined result:**
        $12 + 1 = 13 \pmod{17}$


4.  **Comparison:**
    The Verifier's calculated result is **13**.
    The Prover's provided $f_1(4)$ value is also **13**.

    They match perfectly! This shows that, at least at $x_0=2$, the Prover's first folding calculation is honest.

### Verifying Second Folding: From Linear to Constant Polynomial

Now we need to verify the correctness of the second folding, i.e., the process from $f_1(y) = 7y + 2$ folding to constant polynomial $f_2(z) = 6$.

Assume the Verifier randomly chose $y_0 = 4$ for the second query.

1.  **Verifier's Request:**
    The Verifier will ask the Prover for the following values:
    - $f_1(4) = 13$
    - $f_1(-4) = f_1(13)$ (since $-4 \equiv 13 \pmod{17}$)
    - $f_2(4^2) = f_2(16) = f_2(-1) = 6$ (constant polynomial has value 6 at any point)

2.  **Calculate $f_1(-4) = f_1(13)$:**
    $f_1(13) = 7(13) + 2 = 91 + 2 = 93 \equiv 8 \pmod{17}$ (since $93 = 5 \times 17 + 8$)

3.  **Prover's Response:**
    *   $f_1(4) = 13$
    *   $f_1(-4) = f_1(13) = 8$
    *   $f_2(16) = 6$

4.  **Verifier's Verification Calculation:**
    For the second folding, we have $f_1(y) = 7y + 2$, decomposed as:
    - Even part (constant term): $g_1(z) = 2$
    - Odd part coefficient: $h_1(z) = 7$
    
    The verification relation is:
    $$f_2(y_0^2) = g_1(y_0^2) + \alpha_2 \cdot h_1(y_0^2)$$
    
    But for linear polynomials, we can use a more direct formula:
    $$f_2(y_0^2) = \frac{f_1(y_0) + f_1(-y_0)}{2} + \alpha_2 \cdot \frac{f_1(y_0) - f_1(-y_0)}{2y_0}$$

    Substituting values:
    *   **First part (even part contribution):**
        $\frac{f_1(4) + f_1(-4)}{2} = \frac{13 + 8}{2} = \frac{21}{2}$
        Calculate $\frac{21}{2} = 21 \times 9 = 189 \equiv (11 \times 17 + 2) \equiv 2 \pmod{17}$

    *   **Second part (odd part contribution):**
        $\alpha_2 \cdot \frac{f_1(4) - f_1(-4)}{2y_0} = 3 \cdot \frac{13 - 8}{2(4)} = 3 \cdot \frac{5}{8}$
        We need to compute $8^{-1} \pmod{17}$. Since $15 \times 8 = 120 \equiv 1 \pmod{17}$, so $8^{-1} \equiv 15 \pmod{17}$.
        So, $\frac{5}{8} = 5 \times 15 = 75 \equiv (4 \times 17 + 7) \equiv 7 \pmod{17}$.
        The entire second part equals $3 \times 7 = 21 \equiv 4 \pmod{17}$.

    *   **Combined result:**
        $2 + 4 = 6 \pmod{17}$

5.  **Comparison:**
    The Verifier's calculated result is **6**.
    The Prover's provided $f_2(16)$ value is also **6**.

    They match perfectly! This shows the second folding is also honest.

### Final Verification

Since we have reached constant polynomial $f_2(z) = 6$, the Verifier can directly check this constant value. A constant polynomial should have the same value at all points, which is very easy to verify.

**Complete FRI Verification Summary:**
1. ✅ First folding verification passed: $f(x) \rightarrow f_1(y)$
2. ✅ Second folding verification passed: $f_1(y) \rightarrow f_2(z) = 6$
3. ✅ Final constant polynomial verification: $f_2(z) = 6$ is a valid constant

Through this complete FRI process, the Verifier can be highly confident that the Prover's provided original commitment $C_0$ indeed represents the evaluations of a low-degree polynomial (degree less than 8) without checking all points.


---

## Supplement: Complete Derivation of FRI Folding Core Relation

### Step 1: Even-Odd Decomposition of Polynomials

Any polynomial $P(x)$ can be uniquely decomposed into even and odd parts:

$$P(x) = P_{even}(x) + P_{odd}(x)$$

Where:
- **Even part** $P_{even}(x)$: contains only even-degree terms, can be written as $G(x^2)$
- **Odd part** $P_{odd}(x)$: contains only odd-degree terms, can be written as $x \cdot H(x^2)$

Therefore:
$$P(x) = G(x^2) + x \cdot H(x^2)$$

**Why this decomposition?**
- Even-degree terms have the same values at $x$ and $-x$
- Odd-degree terms have opposite values at $x$ and $-x$
- This symmetry is the core of FRI folding

### Step 2: Establish System of Equations

Substituting $-x$ into the same formula:

$$P(-x) = G((-x)^2) + (-x) \cdot H((-x)^2)$$
$$P(-x) = G(x^2) - x \cdot H(x^2)$$

Now we have a system of two linear equations in $G(x^2)$ and $H(x^2)$:

$$\begin{cases}
P(x) = G(x^2) + x \cdot H(x^2) \quad \text{...(1)} \\
P(-x) = G(x^2) - x \cdot H(x^2) \quad \text{...(2)}
\end{cases}$$

### Step 3: Solve for Even and Odd Parts

Through simple addition and subtraction, we can **uniquely solve** for $G(x^2)$ and $H(x^2)$:

**Solve for even part:** $(1) + (2)$
$$P(x) + P(-x) = 2 \cdot G(x^2)$$
$$\Rightarrow G(x^2) = \frac{P(x) + P(-x)}{2}$$

**Solve for odd part:** $(1) - (2)$
$$P(x) - P(-x) = 2x \cdot H(x^2)$$
$$\Rightarrow H(x^2) = \frac{P(x) - P(-x)}{2x}$$

### Step 4: Introduce Random Challenge Value

In the FRI protocol, the Verifier provides a random challenge value $\alpha$. The Prover uses this challenge value to construct a new polynomial:

$$P_{next}(x^2) = G(x^2) + \alpha \cdot H(x^2)$$

**Why do we need challenge value $\alpha$?**
1. **Prevent cheating**: Without $\alpha$, the Prover might provide fake $G$ and $H$
2. **Randomness**: The randomness of $\alpha$ ensures the Prover cannot prepare fake proofs in advance
3. **Uniqueness**: Each random $\alpha$ produces a different linear combination

### Step 5: Derive FRI Folding Core Relation

Substituting the previous results:

$$P_{next}(x^2) = \frac{P(x) + P(-x)}{2} + \alpha \cdot \frac{P(x) - P(-x)}{2x}$$

This is the **FRI folding core relation**!

### Step 6: Mathematical Meaning of Verification Process

**Verifier's Logic:**
1. If the Prover is honest, then the provided $P(x)$, $P(-x)$ and $P_{next}(x^2)$ should satisfy the above relation
2. Through random sampling, the Verifier can detect dishonest Provers with high probability
3. After multiple rounds of folding, we eventually get a constant polynomial, which is very easy to verify

**Key Insights:**
- Each folding halves the polynomial degree
- Domain size also halves
- But verification security does not decrease (due to random challenge values)

### Example Verification

Using our example, first folding:
- $P(x) = x^3 + 2x^2 + 3x + 4$
- At point $x = 2$: $P(2) = 9$, $P(-2) = 15$
- Challenge value: $\alpha = 5$

Calculate:
$$P_{next}(4) = \frac{9 + 15}{2} + 5 \cdot \frac{9 - 15}{2 \times 2} = 12 + 1 = 13$$

This exactly matches our direct calculation $f_1(4) = 7 \times 4 + 2 = 30 \equiv 13 \pmod{17}$!

---