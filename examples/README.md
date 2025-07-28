# Plonky3 Examples Execution Guide

Welcome to the Plonky3 examples collection! This directory contains multiple Zero-Knowledge Proof implementation examples and educational materials to help you deeply understand the working principles of the Plonky3 framework.

## 📚 Project Overview

Plonky3 is a high-performance zero-knowledge proof framework. This example collection includes:
- **Implementation Examples**: Executable program examples demonstrating different proof scenarios
- **Educational Materials**: Complete learning path from basic theory to advanced implementation
- **Theoretical Explanations**: In-depth mathematical derivations and concept explanations

## 🛠️ Environment Requirements

Before getting started, please ensure your system has the following installed:

1. **Rust Programming Environment**
   ```bash
   # Check Rust version (latest stable version recommended)
   rustc --version
   cargo --version
   ```

2. **Git Version Control**
   ```bash
   git --version
   ```

If Rust is not yet installed, please visit [https://rustup.rs/](https://rustup.rs/) for installation.

## 🚀 Quick Start

### Running Example Programs

All example programs are located in the `examples/examples/` directory and can be executed using the following commands:

```bash
# Basic execution format
cargo run --release --example <example_name>

# Example: Run the Fibonacci prover
cargo run --release --example my_fibonacci
```

> **Note**: Using the `--release` flag significantly improves proof generation performance.

## 📋 Available Examples

### 1. Fibonacci Sequence Prover (`my_fibonacci`)
```bash
cargo run --release --example my_fibonacci
```
**Function**: Proves correct calculation of Fibonacci sequences
**Learning Points**:
- Basic AIR (Algebraic Intermediate Representation) design
- Execution Trace generation
- Implementation of initial constraints and transition constraints

### 2. Universal Adder (`my_adder`)
```bash
cargo run --release --example my_adder
```
**Function**: Proves correct execution of a series of addition instructions
**Learning Points**:
- State management and register operations
- Usage of Selectors
- Algebraic representation of conditional logic

### 3. Arithmetic Logic Unit (`my_alu`)
```bash
cargo run --release --example my_alu
```
**Function**: Advanced processor supporting addition and subtraction operations
**Learning Points**:
- Opcode handling
- Multi-instruction type proof systems
- Implementation of conditional constraints

## 📖 Educational Materials

### Lesson 1: FRI Fundamentals (`lesson1-fri-fundamental-and-example/`)
**Content**:
- Detailed explanation of FRI (Fast Reed-Solomon Interactive Oracle Proofs) protocol
- Complete mathematical derivations and manual calculation examples
- In-depth analysis of the folding process

**Reading Order**:
1. `README.md` - Main theoretical content
2. `README1.md` - Supplementary explanations

### Lesson 2: Plonky3 Concepts (`lesson2-plonky3-concept/`)
**Content**: Core concepts and architecture of the Plonky3 framework

### Lesson 3: Fibonacci Prover Implementation (`lesson3-fibonacci-prover/`)
**Content**:
- Implementing a Fibonacci prover from scratch
- Detailed guidance on AIR design
- Implementation steps and verification methods

**Recommended pairing**: `my_fibonacci` example

### Lesson 4: Universal Adder Implementation (`lesson4-universal-adder/`)
**Content**:
- State machine design and implementation
- Application of selector mechanisms
- Proof methods for instruction processing

**Recommended pairing**: `my_adder` example

### Lesson 5: ALU Implementation (`lesson5-adder-subtractor-alu/`)
**Content**:
- Handling multiple instruction types
- Design of opcode selectors
- Algebraic implementation of conditional logic

**Recommended pairing**:
- `my_alu` example
- `ALU_實作報告.md` - Detailed implementation report

## 🎯 Recommended Learning Path

### Beginner Path
1. **Theoretical Foundation**: Read `lesson1` to understand the FRI protocol
2. **Concept Understanding**: Learn Plonky3 concepts in `lesson2`
3. **Implementation Introduction**: Follow `lesson3` to implement the Fibonacci prover
4. **Execution Verification**: Run the `my_fibonacci` example

### Advanced Path
1. **State Management**: Learn `lesson4` and implement the universal adder
2. **Instruction Processing**: Learn `lesson5` and implement the ALU
3. **Practical Application**: Run all example programs
4. **Deep Research**: Read implementation reports and thinking questions

## 🔧 Troubleshooting

### Common Issues

1. **Compilation Errors**
   ```bash
   # Clean and recompile
   cargo clean
   cargo build --release
   ```

2. **Slow Execution**
   - Ensure using the `--release` flag
   - Check if system memory is sufficient

3. **Example Not Found**
   ```bash
   # Confirm current location is in Plonky3 root directory
   pwd
   # List all available examples
   ls examples/examples/
   ```

### Performance Recommendations

- **Memory**: At least 8GB RAM recommended
- **Compilation**: Always use `--release` mode
- **Parallelism**: Take advantage of multi-core processors

## 💡 Advanced Usage

### Custom Examples
1. Create new `.rs` files in the `examples/examples/` directory
2. Implement your proof logic
3. Execute with `cargo run --release --example <your_filename>`

### Debugging Tips
```bash
# View detailed output
RUST_LOG=debug cargo run --release --example my_fibonacci

# View compilation process
cargo build --release --example my_fibonacci --verbose
```

## 📚 Further Learning

### Related Resources
- [Plonky3 Official Documentation](https://github.com/Plonky3/Plonky3)
- [STARK Protocol Principles](https://starkware.co/stark/)
- [FRI Protocol Paper](https://eccc.weizmann.ac.il/report/2017/134/)

### Suggested Exercises
1. Modify existing examples to add new features
2. Implement different computational logic
3. Optimize proof generation performance
4. Explore different finite field applications

## 🤝 Contribution Guidelines

Welcome to submit issue reports, feature suggestions, or code improvements! Please refer to the main project's contribution guidelines.

## 📄 License Information

This project adopts the same license terms as Plonky3. For detailed information, please refer to the LICENSE file in the root directory.

---

**Happy learning! If you have any questions, feel free to check the educational materials or submit an Issue.**
