# Structure and Interpretation of Computer Program in Rust

## Table of Content

[Chapter 1. Building Abstractions with Functions](./src/build_abstraction_with_function)

- [1.1 The Elements of Programming](./src/build_abstraction_with_function/elements_of_programming.rs)
- [1.2 Functions and the Processes They Generate](./src/build_abstraction_with_function/functions_and_their_processes.rs)
- 1.3 Formulating Abstractions with Higher-Order Functions
  - [Function as General Method](./src/build_abstraction_with_function/function_as_general_method.rs)
  - [High Order Functions](./src/build_abstraction_with_function/high_order_functions.rs)
  - [Newton's Method](./src/build_abstraction_with_function/newton_method.rs)

[Chapter 2. Building Abstractions with Data](./src/build_abstraction_with_data)

- [2.1 Introduction to Data Abastraction](./src/build_abstraction_with_data/data_abstraction.rs)
- 2.2 Hierarchical Data and the Closure Property
- 2.3 Symbolic Data
- 2.4 Multiple Representations for Abstract Data
- 2.5 Systems with generic Operations

Chapter 3. Modularity, Objects, and State

Chapter 4. Metalinguistic Abstraction

Chapter 5. Computing with Register Machines



## edit and compile

'''
cargo watch
'''

## edit and test

'''
cargo watch -x test
// run with nocapture flag for particular test
cargo watch -x "test -- --nocapture functions_and_their_processes_tests""
'''
