# ckks-engine

`ckks-engine` is a Rust crate that provides an implementation of the CKKS (Cheon-Kim-Kim-Song) homomorphic encryption scheme. This enables encrypted computations on real numbers and strings while preserving the privacy of the underlying data. With `ckks-engine`, you can perform a wide range of mathematical operations on encrypted data, including addition, subtraction, multiplication, and more.

## CKKS Scheme Overview

The CKKS scheme is a homomorphic encryption method designed to allow for computations on encrypted data with a certain level of error tolerance. It is particularly well-suited for applications that require the processing of real numbers while maintaining confidentiality. The CKKS scheme utilizes polynomial encoding to represent real numbers and supports operations directly on these encoded values.

### Version 1.0 Release

We are excited to announce the initial release of version 1.0 of `ckks-engine`. This version includes a complete implementation of the CKKS homomorphic encryption algorithm, along with essential features designed for secure and efficient encrypted computations. Key highlights of this release include:

- **CKKS Algorithm Implementation**: The complete implementation of the CKKS homomorphic encryption algorithm, allowing for secure encryption and computation of real numbers.

- **Encryption and Decryption Functions**: Secure functions to encrypt and decrypt data using the CKKS scheme, ensuring data privacy throughout the process.

- **Basic Arithmetic Operations**:
  - **Addition**: Perform encrypted addition, allowing for the summation of encrypted values without revealing the underlying data.
  - **Subtraction**: Execute encrypted subtraction to determine the difference between encrypted values.
  - **Multiplication**: Multiply encrypted values securely, enabling more complex computations while preserving privacy.
  - **Negation**: Compute the negation of encrypted values, providing additional flexibility in mathematical operations.

## Usage

To test and utilize the features of `ckks-engine`, a sample code is provided in `main.rs`. This code demonstrates the functionality of the encryption scheme and basic arithmetic operations.

### Instructions to Run the Code

1. Clone the repository:
   ```bash
   git clone https://github.com/Chandan-M-N/ckks-engine.git
    ```
2. Navigate to the project directory:
    ```bash
    cd ckks-engine
    ```
3. Build the project using cargo:
    ```bash
    cargo build
    ```
4. Run the sample code:
    ```bash
    cargo run
    ```

This will execute the sample code in main.rs, illustrating how to use the ckks-engine crate for encrypted computations.

## Conclusion
The ckks-engine crate empowers developers to perform secure computations on real numbers using the CKKS homomorphic encryption scheme. We invite you to explore the features of this crate and integrate it into your projects where data privacy is of utmost importance.
