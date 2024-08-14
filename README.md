This Rust program demonstrates how to improve the performance of fully homomorphic encryption (FHE) operations by employing parallel processing. Using the Rayon library, this code enables efficient parallel computing of homomorphic additions with the TFHE-rs library. The code improves the TFHE-rs library's performance while working with parallelizable tasks, especially on systems with multiple cores. 
## Overview
Fully Homomorphic Encryption (FHE) enables computations on encrypted data without having to decrypt it. However, these methods can be computationally expensive, particularly for huge datasets. This project uses Rayon, a Rust data-parallelism package, to add parallelism to homomorphic operations.
## Features
- **Parallel Homomorphic Addition:** Perform homomorphic addition operations in parallel using Rayon, significantly improving performance on multi-core systems.
- **Custom Trait for Reusability:** Encapsulates parallel addition in a reusable trait, making it easy to extend to other homomorphic operations.
- **Secure Computation:** Maintains the security properties of FHE while improving computational efficiency.

## Performance Enhancement Through Parallelism
- Parallel Processing:
   - By leveraging the Rayon library to perform operations such as addition in parallel, the code can minimize the time necessary to handle massive datasets of encrypted information.
   - This is especially useful when many homomorphic procedures must be done concurrently, as parallel processing can split the workload across multiple CPU cores.
- Efficiency in Multi-Core Systems:
   - In a multi-core context, parallel execution of homomorphic procedures can result in significant performance benefits, especially when working with large encrypted data vectors.
   - This reduces the time complexity from linear (in a single-threaded case) to more favorable, depending on the number of cores available and the nature of the operation.
## Extends Functionality at the User Level:
- This code introduces a custom trait, ParallelHomomorphicOps, which enables parallel operations on FheUint64 encrypted values.
- It wraps the logic for parallel addition, making it more accessible and convenient for the user.
- Users can now perform parallel homomorphic operations without needing to manually handle the parallelism each time.
## Requirements
- **Rust:** Ensure you have the latest stable version of Rust installed.
- **Rust version:** a minimum Rust version of $1.73$ is required to compile TFHE-rs.
- **TFHE-rs:** This project uses the TFHE-rs library for fully homomorphic encryption. It is included as a dependency in the Cargo.toml file.
- **Rayon:** The Rayon library is used for parallel processing, also included as a dependency.
``` 
#To include library run:
cargo add tfhe

#Alternatively paste the line below in 'Cargo.toml' 
#For x86_64 machine running a Unix-like OS:

tfhe = { version = "0.7.2", features = [ "boolean", "shortint", "integer", "x86_64-unix" ] }
#For ARM machine running a Unix-like OS:

tfhe = { version = "0.7.2", features = [ "boolean", "shortint", "integer", "aarch64-unix" ] }
#For x86_64 machines with the rdseed instruction running Windows:

tfhe = { version = "*", features = ["boolean", "shortint", "integer", "x86_64"] }

#ensure to build cargo after adding the tfhe library
cargo run build
```
## Usage 
- The project includes a simple example demonstrating parallel homomorphic addition:
   - **Main Function:** The entry point of the application, which calls the parallel_encryption_addition function.
   - **Parallel Addition:** The parallel_encryption_addition function encrypts two 64-bit integers, adds them in parallel, and then decrypts the result to verify correctness.
## How It Works
- **Encryption and decryption:** The code uses the TFHE-rs library to encrypt 64-bit integers, then performs homomorphic addition on the encrypted data and decrypts the result.
- **Parallel Addition:** FheUint64 implements the ParallelHomomorphicOps trait, which allows for parallel addition using Rayon.
## Performance Considerations
- **Multi-Core Advantage:** Parallel addition will result in significant performance gains for systems with multiple CPU cores, particularly for large datasets.
- **Overhead for Small Data:** For small datasets, the overhead of parallelism might outweigh the benefits. Consider employing  single-threaded operations for small-scale computations.
## Run code
>[!TIP]
> Performance: for optimal performance, it is highly recommended to run code that uses TFHE-rs in release mode with cargo's --release flag.
>```
>cargo -- run release
>```
 ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.

```bash
git clone https://github.com/cypriansakwa/Parallel_Homomorphic_Operations_for_TFHE_using_Rayon.git
cd Parallel_Homomorphic_Operations_for_TFHE_using_Rayon
