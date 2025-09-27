use tsp::{
    tsp,
    extract_onchain_proof_data
};
use tsp_methods::TSP_ID;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let n = 4;
    let distances = vec![
        0, 10, 15, 20,
        10, 0, 35, 25,
        15, 35, 0, 30,
        20, 25, 30, 0,
    ];
    let tour = vec![0, 2, 3, 1];

    let (receipt, total) = tsp(n, distances, tour.clone());

    // Verify locally first
    receipt.verify(TSP_ID).expect(
        "Local proof verification failed; check you used the correct image ID.",
    );
    println!("Locally verified tour {:?} with total length {}", tour, total);

    let (seal, journal, journal_digest) = extract_onchain_proof_data(&receipt);
    
    // Generate TSP_ID in different byte order formats
    
    // 1. Little-endian (current format)
    let tsp_id_le: Vec<u8> = TSP_ID
        .iter()
        .flat_map(|&x| x.to_le_bytes())
        .collect();
    
    // 2. Big-endian
    let tsp_id_be: Vec<u8> = TSP_ID
        .iter()
        .flat_map(|&x| x.to_be_bytes())
        .collect();
    
    // 3. Reversed little-endian (reverse the array order, then little-endian bytes)
    let tsp_id_rev_le: Vec<u8> = TSP_ID
        .iter()
        .rev()
        .flat_map(|&x| x.to_le_bytes())
        .collect();
    
    // 4. Reversed big-endian (reverse the array order, then big-endian bytes)
    let tsp_id_rev_be: Vec<u8> = TSP_ID
        .iter()
        .rev()
        .flat_map(|&x| x.to_be_bytes())
        .collect();
    
    println!("On-chain proof components:");
    println!("- Image ID (TSP_ID): {:?}", TSP_ID);
    println!("- Little-endian:     0x{}", hex::encode(&tsp_id_le));
    println!("- Big-endian:        0x{}", hex::encode(&tsp_id_be));
    println!("- Reversed LE:       0x{}", hex::encode(&tsp_id_rev_le));
    println!("- Reversed BE:       0x{}", hex::encode(&tsp_id_rev_be));
    println!("- Seal length: {} bytes", seal.len());
    println!("- Journal digest: {}", hex::encode(journal_digest));

    // Save proof components to files for on-chain submission
    std::fs::write("proof_seal.bin", &seal).unwrap();
    println!("   Seal: 0x{}", hex::encode(&seal));
    std::fs::write("proof_journal.bin", &journal).unwrap();
    std::fs::write("proof_digest.txt", hex::encode(journal_digest)).unwrap();
    
    // Save TSP_ID in all byte order formats
    
    // Little-endian (default)
    std::fs::write("tsp_id_le.bin", &tsp_id_le).unwrap();
    std::fs::write("tsp_id_le.txt", hex::encode(&tsp_id_le)).unwrap();
    
    // Big-endian
    std::fs::write("tsp_id_be.bin", &tsp_id_be).unwrap();
    std::fs::write("tsp_id_be.txt", hex::encode(&tsp_id_be)).unwrap();
    
    // Reversed little-endian
    std::fs::write("tsp_id_rev_le.bin", &tsp_id_rev_le).unwrap();
    std::fs::write("tsp_id_rev_le.txt", hex::encode(&tsp_id_rev_le)).unwrap();
    
    // Reversed big-endian
    std::fs::write("tsp_id_rev_be.bin", &tsp_id_rev_be).unwrap();
    std::fs::write("tsp_id_rev_be.txt", hex::encode(&tsp_id_rev_be)).unwrap();
    
    // Keep original files for backward compatibility
    std::fs::write("tsp_id.bin", &tsp_id_le).unwrap();    
    std::fs::write("tsp_id.txt", hex::encode(&tsp_id_le)).unwrap();
    
    println!("Proof files saved for on-chain verification:");
    println!("  - proof_seal.bin");
    println!("  - proof_journal.bin");
    println!("  - proof_digest.txt");
    println!("  TSP_ID formats:");
    println!("  - tsp_id_le.bin/txt (little-endian)");
    println!("  - tsp_id_be.bin/txt (big-endian)");
    println!("  - tsp_id_rev_le.bin/txt (reversed little-endian)");
    println!("  - tsp_id_rev_be.bin/txt (reversed big-endian)");
    println!("  - tsp_id.bin/txt (default: little-endian)");
}
