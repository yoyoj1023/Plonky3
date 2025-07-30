![Plonky3-powered-by-polygon](https://github.com/Plonky3/Plonky3/assets/86010/7ec356ad-b0f3-4c4c-aa1d-3a151c1065e7)

[![License](https://img.shields.io/badge/License-MIT-green.svg)](https://github.com/Plonky3/Plonky3/blob/main/LICENSE-MIT)
[![License](https://img.shields.io/github/license/Plonky3/Plonky3)](https://github.com/Plonky3/Plonky3/blob/main/LICENSE-APACHE)

Plonky3 is a toolkit which provides a set of primitives, such as polynomial commitment schemes, for implementing polynomial IOPs (PIOPs). It is mainly used to power STARK-based zkVMs, though in principle it may be used for PLONK-based circuits or other PIOPs.

For questions or discussions, please use the Telegram group, [t.me/plonky3](https://t.me/plonky3).

## 🎓 Plonky3 Learning Tutorial - Complete Guide to Zero-Knowledge Proof System Development

### 📚 Tutorial Overview

In the [`Plonky3/examples/`](https://github.com/yoyoj1023/Plonky3/tree/main/examples) directory, we have carefully designed a complete Plonky3 learning course containing **5 progressive exercises** that guide you from zero foundation to mastering zero-knowledge proof system development. Whether you are a cryptography newcomer or a blockchain developer, this tutorial will help you deeply understand the core principles and practical applications of STARK proof systems.

### 🎯 Learning Curve Design

Our learning path adopts a **theory and practice balanced** design philosophy, where each exercise builds upon the previous one:

#### 📖 Stage 1: Theoretical Foundation Building
**[Lesson 1: FRI Fundamentals and Manual Calculation Practice](examples/lesson1-fri-fundamental-and-example/)**
- **Learning Objectives**: Deep understanding of the FRI (Fast Reed-Solomon Interactive Oracle Proof) protocol
- **Core Content**:
  - Mathematical principles of Polynomial Folding
  - Practical applications of finite field arithmetic
  - Verifier and Prover interaction mechanisms
  - Complete manual verification examples (computing f(x) = x³ + 2x² + 3x + 4 in 𝔽₁₇)
- **Skill Enhancement**: Build intuitive understanding of the underlying mathematics of zero-knowledge proofs
- **Target Audience**: Learners who want to deeply understand the operational mechanisms of the FRI protocol

#### 🧠 Stage 2: Conceptual Framework Mastery
**[Lesson 2: Plonky3 Core Concepts Q&A Challenge](examples/lesson2-plonky3-concept/)**
- **Learning Objectives**: Comprehensively master Plonky3's core concepts and terminology
- **Core Content**:
  - Design principles of AIR (Algebraic Intermediate Representation)
  - Generation and verification of Execution Traces
  - Critical role of FRI protocol in STARK
  - Application scenarios for Recursive Proofs
  - Advantages of Plonky3's modular architecture
- **Learning Method**: 10 Q&A questions + 10 true/false questions, covering from basic definitions to advanced applications
- **Skill Enhancement**: Form a complete zero-knowledge proof knowledge system
- **Target Audience**: Developers preparing to deeply learn Plonky3 implementation

#### 💻 Stage 3: Implementation Capability Development
**[Lesson 3: Fibonacci Sequence Prover Implementation](examples/lesson3-fibonacci-prover/)**
- **Learning Objectives**: Master the basic Plonky3 development workflow and implement the first complete proof system
- **Core Content**:
  - Define Plonky3 "Chip" components
  - Generate Execution Traces
  - Implement AIR constraints (initial constraints + transition constraints)
  - Generate and verify STARK proofs
- **Implementation Focus**:
  - Design AIR specification for Fibonacci sequence
  - Handle boundary constraints (F(0)=0, F(1)=1)
  - Implement state transition constraints (F(n) = F(n-1) + F(n-2))
- **Skill Enhancement**: Acquire basic zero-knowledge proof system development capabilities
- **Target Audience**: Programmers with Rust foundation who want to get started with Plonky3 development

#### 🔧 Stage 4: Advanced System Architecture
**[Lesson 4: Universal Adder Processor Implementation](examples/lesson4-universal-adder/)**
- **Learning Objectives**: Learn to handle state management and configurable operations, laying the foundation for ZK-VM construction
- **Core Content**:
  - Design CPU models with multiple registers
  - Use Selectors to implement dynamic operation selection
  - Implement one-hot encoding and constraint verification
  - Handle complex state transition logic
- **Implementation Focus**:
  - State management of 4 registers
  - Complete implementation of ADD instruction
  - Constraint design for selector fields
  - Generate execution traces from instruction sequences
- **Skill Enhancement**: Master zero-knowledge proof design for complex state systems
- **Target Audience**: Advanced developers who want to build virtual machine proof systems

#### 🚀 Stage 5: Advanced System Design
**[Lesson 5: Arithmetic Logic Unit (ALU) Implementation](examples/lesson5-adder-subtractor-alu/)**
- **Learning Objectives**: Master conditional logic and opcode handling, implement complete ALU system
- **Core Content**:
  - Design of Operation Selectors
  - Implementation of Conditional Constraints
  - Unified processing framework for multiple instruction types
  - Modular extension of existing systems
- **Implementation Focus**:
  - Support both ADD and SUB instructions simultaneously
  - Design opcode selection mechanisms
  - Implement conditional algebraic constraints
  - Handle complex state transition logic
- **Skill Enhancement**: Acquire the ability to design complex zero-knowledge virtual machines
- **Target Audience**: Professional developers preparing to develop production-grade ZK-VM systems

### 📈 Learning Outcomes and Capability Advancement

After completing this tutorial, you will be able to:

🎯 **Theoretical Mastery**
- Deep understanding of FRI protocol mathematical principles and implementation details
- Master the complete workflow of STARK proof systems
- Understand the cryptographic foundations of zero-knowledge proofs

🛠️ **Implementation Capabilities**
- Independently design and implement AIR specifications
- Proficiently use the Plonky3 framework to develop proof systems
- Handle complex state management and constraint design

🏗️ **System Design**
- Design scalable zero-knowledge virtual machine architectures
- Implement efficient proof generation and verification processes
- Master modular system design principles

🚀 **Practical Applications**
- Develop ZK-Rollup systems for blockchain scaling solutions
- Implement privacy-preserving computational verification schemes
- Build high-performance zero-knowledge proof infrastructure

### 🎓 Getting Started

We recommend following the learning sequence: Lesson 1 → Lesson 2 → Lesson 3 → Lesson 4 → Lesson 5. Each exercise contains detailed documentation, implementation guidance, and reflection questions to ensure you can solidly grasp every concept.

📝 **Learning Recommendations**:
- For theoretical exercises, please calculate by hand to build mathematical intuition
- For programming exercises, please write code yourself and avoid direct copying
- After completing each exercise, consider possibilities for extended applications
- Recommend maintaining learning notes to record important concepts and implementation details


## Status

Fields:
- [x] Mersenne31
  - [x] "complex" extension field
  - [x] ~128 bit extension field
  - [x] AVX2
  - [x] AVX-512
  - [x] NEON
- [x] General 31 bit fields (BabyBear and KoalaBear)
  - [x] ~128 bit extension field
  - [x] AVX2
  - [x] AVX-512
  - [x] NEON
- [x] Goldilocks
  - [x] ~128 bit extension field

Generalized vector commitment schemes
- [x] generalized Merkle tree

Polynomial commitment schemes
- [x] FRI-based PCS
- [ ] tensor PCS
- [ ] univariate-to-multivariate adapter
- [ ] multivariate-to-univariate adapter

PIOPs
- [x] univariate STARK
- [ ] multivariate STARK
- [ ] PLONK

Codes
- [x] Brakedown
- [x] Reed-Solomon

Interpolation
- [x] Barycentric interpolation
- [x] radix-2 DIT FFT
- [x] radix-2 Bowers FFT
- [ ] four-step FFT
- [x] Mersenne circle group FFT

Hashes
- [x] Rescue
- [x] Poseidon
- [x] Poseidon2
- [x] BLAKE3
  - [ ] modifications to tune BLAKE3 for hashing small leaves
- [x] Keccak-256
- [x] Monolith


## Benchmarks

Many variations are possible, with different fields, hashes, and so forth, which can be controlled through the command line.

For example, to prove 2^20 Poseidon2 permutations of width 16, using the `KoalaBear` field, `Radix2DitParallel` DFT, and `KeccakF` as the Merkle tree hash:
```bash
RUSTFLAGS="-Ctarget-cpu=native" cargo run --example prove_prime_field_31 --release --features parallel -- --field koala-bear --objective poseidon-2-permutations --log-trace-length 17 --discrete-fourier-transform radix-2-dit-parallel --merkle-hash keccak-f
```

Currently the options for the command line arguments are:
- `--field` (`-f`): `mersenne-31` or `koala-bear` or `baby-bear`.
- `--objective` (`-o`): `blake-3-permutations, poseidon-2-permutations, keccak-f-permutations`.
- `--log-trace-length` (`-l`): Accepts any integer between `0` and `255`. The number of permutations proven is `trace_length, 8*trace_length` and `trace_length/24` for `blake3, poseidon2` and `keccakf` respectively. 
- `--discrete-fourier-transform` (`-d`): `radix-2-dit-parallel, recursive-dft`. This option should be omitted if the field choice is `mersenne-31` as the circle stark currently only supports a single discrete fourier transform.
- `--merkle-hash` (`-m`): `poseidon-2, keccak-f`.

Extra speedups may be possible with some configuration changes:
- `JEMALLOC_SYS_WITH_MALLOC_CONF=retain:true,dirty_decay_ms:-1,muzzy_decay_ms:-1` will cause jemalloc to hang on to virtual memory. This may not affect the very first proof much, but can help significantly with subsequent proofs as fewer pages (if any) will need to be newly assigned by the OS. These settings might not be suitable for all production environments, e.g. if the process' virtual memory is limited by `ulimit` or `max_map_count`.
- Adding `lto = "fat"` in the top-level `Cargo.toml` may improve performance slightly, at the cost of longer compilation times.

## CPU features

Plonky3 contains optimizations that rely on newer CPU instructions unavailable in older processors. These instruction sets include x86's [BMI1 and 2](https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set), [AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#Advanced_Vector_Extensions_2), and [AVX-512](https://en.wikipedia.org/wiki/AVX-512). Rustc does not emit those instructions by default; they must be explicitly enabled through the `target-feature` compiler option (or implicitly by setting `target-cpu`). To enable all features that are supported on your machine, you can set `target-cpu` to `native`. For example, to run the tests:
```bash
RUSTFLAGS="-Ctarget-cpu=native" cargo test
```

Support for some instructions, such as AVX-512, is still experimental. They are only available in the nightly build of Rustc and are enabled by the [`nightly-features` feature flag](#nightly-only-optimizations). To use them, you must enable the flag in Rustc (e.g. by setting `target-feature`) and you must also enable the `nightly-features` feature.


## Nightly-only optimizations

Some optimizations (in particular, AVX-512-optimized math) rely on features that are currently available only in the nightly build of Rustc. To use them, you need to enable the `nightly-features` feature. For example, to run the tests:
```bash
cargo test --features nightly-features
```


## Known issues

The verifier might panic upon receiving certain invalid proofs.


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Guidance for external contributors

Do you feel keen and able to help with Plonky3? That's great! We
encourage external contributions!

We want to make it easy for you to contribute, but at the same time, we
must manage the burden of reviewing external contributions. We are a
small team, and the time we spend reviewing external contributions is
time we are not developing ourselves.

We also want to help you avoid inadvertently duplicating work that
is already underway, or building something that we will not
want to incorporate.

First and foremost, please keep in mind that this is a highly
technical piece of software and contributing is only suitable for
experienced mathematicians, cryptographers, and software engineers.

The Polygon Zero Team reserves the right to accept or reject any
external contribution for any reason, including a simple lack of time
to maintain it (now or in the future); we may even decline to review
something that is not considered a sufficiently high priority for us.

To avoid disappointment, please communicate your intention to
contribute openly, while respecting the limited time and availability
we have to review and provide guidance for external contributions. It
is a good idea to drop a note in our public Discord #development
channel about your intention to work on something, whether an issue, a
new feature, or a performance improvement. This is probably all that's
really required to avoid duplication of work with other contributors.

What follows are some more specific requests for how to write PRs in a
way that will make them easy for us to review. Deviating from these
guidelines may result in your PR being rejected, ignored, or forgotten.


### General guidance for your PR

Obviously, PRs will not be considered unless they pass our Github
CI. The GitHub CI is not executed for PRs from forks, but you can
simulate the GitHub CI by running the commands in
`.github/workflows/ci.yml`.

Under no circumstances should a single PR mix different purposes: Your
PR is either a bug fix, a new feature, or a performance improvement,
never a combination. Nor should you include, for example, two
unrelated performance improvements in one PR. Please just submit
separate PRs. The goal is to make reviewing your PR as simple as
possible, and you should be thinking about how to compose the PR to
minimize the burden on the reviewer.

Plonky3 uses stable Rust, so any PR that depends on unstable features
is likely to be rejected. It's possible that we may relax this policy
in the future, but we aim to minimize the use of unstable features;
please discuss with us before enabling any.

Please do not submit any PRs consisting of merely minor fixes to documentation.
They will not be accepted. If you find something that bothers you particularly,
make an issue and we will fix it when we find the time.

Here are a few specific guidelines for the three main categories of
PRs that we expect:


#### The PR fixes a bug

In the PR description, please clearly but briefly describe

1. the bug (could be a reference to a GH issue; if it is from a
   discussion (on Discord/email/etc. for example), please copy in the
   relevant parts of the discussion);
2. what turned out to cause the bug; and
3. how the PR fixes the bug.

Wherever possible, PRs that fix bugs should include additional tests
that (i) trigger the original bug and (ii) pass after applying the PR.


#### The PR implements a new feature

If you plan to contribute the implementation of a new feature, please
double-check with the Polygon Zero team that it is a sufficient
priority for us that it will be reviewed and integrated.

In the PR description, please clearly but briefly describe

1. what the feature does
2. the approach taken to implement it

All PRs for new features must include a suitable test suite.


#### The PR improves performance

Performance improvements are particularly welcome! Please note that it
can be quite difficult to establish true improvements for the
workloads we care about. To help filter out false positives, the PR
description for a performance improvement must clearly identify

1. the target bottleneck (only one per PR to avoid confusing things!)
2. how performance is measured
3. characteristics of the machine used (CPU, OS, #threads if appropriate)
4. performance before and after the PR


### Licensing

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
