use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint64};
use rayon::prelude::*;

// Define a trait to encapsulate parallel homomorphic operations
pub trait ParallelHomomorphicOps {
    fn parallel_add(self, other: Vec<Self>) -> Vec<Self>
    where
        Self: Sized + Clone + Send + Sync;
}

impl ParallelHomomorphicOps for FheUint64 {
    fn parallel_add(self, other: Vec<Self>) -> Vec<Self> {
        let self_vec = vec![self; other.len()]; // Replicate self to match the size of other vector
        self_vec
            .into_par_iter()
            .zip(other.into_par_iter())
            .map(|(x, y)| x + y)
            .collect()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    parallel_encryption_addition()?;
    Ok(())
}

fn parallel_encryption_addition() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration using small encryption parameters
    let config = ConfigBuilder::default_with_small_encryption();

    // Generate client and server keys
    let (client_key, server_key) = generate_keys(config);

    // Set server key for encrypted operations
    set_server_key(server_key);

    // Clear text values to be encrypted
    let clear_a = 1234567890123456789_u64;
    let clear_b = 9876543210987654321_u64;

    // Encrypt the clear text values
    let a = FheUint64::try_encrypt(clear_a, &client_key)?;
    let b = FheUint64::try_encrypt(clear_b, &client_key)?;

    // Use the custom parallel addition operation
    let result = a.parallel_add(vec![b]);

    // Decrypt the result
    let decrypted_result: Vec<u64> = result
        .into_iter()
        .map(|c| c.decrypt(&client_key))
        .collect();

    // Clear text addition for verification
    let clear_result = clear_a + clear_b;
    assert_eq!(decrypted_result[0], clear_result);

    println!("Decrypted result: {:?}", decrypted_result[0]);

    Ok(())
}

