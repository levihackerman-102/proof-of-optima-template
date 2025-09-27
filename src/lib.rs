use tsp_methods::TSP_ELF; // still the guest ELF
use risc0_zkvm::{default_prover, sha::Digestible, Digest, ExecutorEnv, Prover, ProverOpts, Receipt};

/// Run the TSP verifier guest.
/// - `n`: number of cities
/// - `distances`: flattened distance matrix (n*n entries)
/// - `tour`: sequence of city indices (length n)
pub fn tsp(n: usize, distances: Vec<u64>, tour: Vec<usize>) -> (Receipt, u64) {
    assert_eq!(distances.len(), n * n, "distance matrix must be n*n");
    assert_eq!(tour.len(), n, "tour must have n cities");

    // Build executor env with inputs in the order the guest expects
    let mut env_builder = ExecutorEnv::builder();
    env_builder.write(&n).unwrap();
    for d in &distances {
        env_builder.write(d).unwrap();
    }
    for t in &tour {
        env_builder.write(t).unwrap();
    }
    let env = env_builder.build().unwrap();

    // Prove guest execution
    let prover = default_prover();
    let opts = ProverOpts::groth16();
    let prove_info = prover.prove_with_opts(env, TSP_ELF, &opts).unwrap();
    let receipt = prove_info.receipt;

    // Extract the committed total tour length
    let total: u64 = receipt.journal.decode().unwrap();

    println!("Tour {:?} has total length = {}", tour, total);

    (receipt, total)
}

pub fn extract_onchain_proof_data(receipt: &Receipt) -> (Vec<u8>, Vec<u8>, Digest) {
    // Extract the Groth16 seal (proof)
    let seal = receipt.inner.groth16().unwrap().seal.clone();
    
    // Get journal bytes
    let journal = receipt.journal.bytes.clone();

    // Compute journal digest (SHA256 hash)
    let journal_digest = journal.digest();
    
    (seal, journal, journal_digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsp() {
        let n = 4;
        let distances = vec![
            0, 10, 15, 20,
            10, 0, 35, 25,
            15, 35, 0, 30,
            20, 25, 30, 0,
        ];
        let tour = vec![0, 2, 3, 1];
        let (_, total) = tsp(n, distances, tour.clone());

        assert_eq!(total, 80, "expected tour length is 80");
    }
}
