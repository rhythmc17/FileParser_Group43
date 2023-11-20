# File Parser

## Problem statement

The "file_parser" project is a CSV parsing tool designed for simplicity, flexibility, and efficiency. This README provides an overview of the project, its architecture, adherence to Principles of Programming Languages (PoPL), challenges faced during development, and potential avenues for future work.

## PoPL Angle:

### Abstraction for Readability:

Incorporate basic abstraction principles for a clean and modular codebase, promoting readability and ease of maintenance.

### Flexibility in Usage:

Leverage the expressiveness of the programming language to allow users to define extraction criteria in a straightforward and flexible manner.

### Graceful Error Handling:

Implement error-handling practices as a core language feature, providing clear and practical error messages for efficient issue resolution.

### Consideration for Efficiency:

Apply practical optimization techniques to ensure efficient performance, especially when dealing with large datasets.

### Differentiation:

The "file_parser" project stands out through:

- **Simplified Abstraction:** Prioritizing a simple and modular design for the CSV parsing library.
- **Pragmatic Error Handling:** Implementing error handling practically for easy troubleshooting.
- **Balanced Optimization:** Striving for efficiency without sacrificing simplicity.

## Software Architecture

### CSV Parser Module (nom Library):

Responsible for defining and executing parsing logic using the Nom parser combinators encapsulated in the `parse_line` function.

### Main Application:

Reads the CSV file, iterates over its lines, and applies the parsing logic. Prints parsed records or error messages to the console.

### Reuse and Development:

- **Reused Components:** Heavily relies on the nom library for parsing, a reusable parser combinator library.
- **Developed Components:** Specific to this use case, including file reading, parsing logic, and error handling.

### Architecture Details:

Standalone command-line application; not a client-server architecture. Testing components (unit and integration tests) are expected in the same or separate modules. No database involvement; focuses on reading and parsing CSV data.

### Architecture Diagram

The architecture of the "file_parser" project can be visualized with the following diagram:

```plaintext
+---------------------+
|  Main Application   |
|---------------------|
|   - Read CSV file   |
|   - Parse lines     |
|   - Print results   |
+----------|----------+
           |
  +--------v--------+
  | CSV Parser (nom)|
  |-----------------|
  |  - Parsing Logic|
  |  - Parser       |
  +-----------------+

```
## PoPL Aspects:

### Parser Combinators:

The use of nom represents the idea of parser combinators, where smaller parsers are combined to create more complex parsers. This is evident in the function parse_line, where different combinators like alphanumeric, tag(","), and tuple are used to create a parser for a CSV line.

### Functional Programming Style:

The code follows a functional programming style, which is often associated with PLT principles. Functions like tag, alphanumeric1, and multispace1 are applied in a functional manner, composing larger functionalities from smaller building blocks.

### Pattern Matching:

The match statement in the main function demonstrates the use of pattern matching, a concept often found in functional languages. This is a common construct in PLT languages for handling different cases or outcomes.

### Immutable Data Structures:

The Record struct is immutable, as it doesn't implement any mutable methods. This aligns with functional programming principles, where immutability is often preferred for easier reasoning about code.

### Result Type for Error Handling:

The use of IResult and the Result type for error handling aligns with the idea of handling errors explicitly. This is a common practice in Rust and also reflects some PLT principles of robust error handling.

### Monomorphic Parsing:

The parser is monomorphic, as it specifically expects alphanumeric fields separated by a comma. This reflects the principle of specifying and enforcing types in a language, contributing to code reliability.


### Error Reporting:
The code provides detailed error reporting with the eprintln!("Error: {:?}", err) statement. This is important for debugging and understanding issues, aligning with PLT principles of clear and informative error messages.

### File I/O Handling:

The code incorporates file I/O operations (fs::read_to_string) to read the CSV data from a file. This touches upon aspects of input and output handling, which are essential considerations in programming language design.

## Challenges Faced:

- While the code demonstrates several aspects related to parsing and functional programming, it's important to note that the code is relatively simple and focused on a specific use case. More complex scenarios might require additional considerations and advanced techniques.

- Debugging parser combinators can be challenging, especially when dealing with complex grammars. The use of macros and the specific syntax of nom can sometimes make it harder to understand and debug parser code.

- Choosing the right combinators and understanding how they interact can be non-trivial, and it often requires a good understanding of the nom library and the specific parsing requirements.

- Error handling in parser combinators can be verbose, and understanding and managing error types can be a learning curve, especially for those new to parser combinators.

- Overall, while the code touches upon some POPL-related concepts, it's important to note that the POPL field is vast and covers a wide range of topics beyond what is explicitly demonstrated in this code snippet.


## Potential for Future Work:

- Enhanced Abstraction Layers: Explore advanced abstraction techniques.
- Advanced Expressiveness: Allow users to define complex extraction criteria.
- Advanced Error Handling Strategies: Investigate comprehensive logging or nuanced error reporting.
- Concurrency and Parallelism: Introduce concurrency for parsing performance optimization.
- Static Analysis for Optimization: Implement static analysis tools for compile-time optimization.
- User-Defined Language Extensions: Allow users to define custom language extensions.
- Integration of Formal Methods: Investigate formal methods for parser correctness.
- Integration of JIT Compilation: Evaluate JIT compilation for dynamic optimization.
- Incorporation of Machine Learning Techniques: Explore machine learning for pattern recognition within CSV files.

---

## How to Run a file 
- ## For C++ code ##: g++ csv_c++.cpp ./a.out
- ## for Rust code ##: cargo build

## Contribution
All the work was done together.

*Note: The provided information serves as a comprehensive guide for understanding and contributing to the "file_parser" project.*
