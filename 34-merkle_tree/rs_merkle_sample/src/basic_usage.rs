use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};

pub fn test() {
    println!("--- Start module: {}", module_path!());

    // Build Merkle tree.
    let leaf_values = ["a", "b", "c", "d", "e", "f"];
    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    // println!("Tree leaves len: {}", merkle_tree.leaves_len());
    // println!("Tree leaves: {:?}", merkle_tree.leaves());
    println!("Tree depth: {}", merkle_tree.depth());

    // Generate proof.
    let indices_to_prove = vec![3, 4];
    let merkle_proof = merkle_tree.proof(&indices_to_prove);

    // Serialize and deserialize the proof.
    let proof_bytes = merkle_proof.to_bytes();
    println!("Proof size: {}", proof_bytes.len());
    let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();

    // Verify the proof.
    let merkle_root = merkle_tree
        .root()
        .ok_or("can't get the merkle root.")
        .unwrap();
    let leaves_to_prove = leaves
        .get(3..5)
        .ok_or("can't get leaves to prove.")
        .unwrap();

    let verified = proof.verify(
        merkle_root,
        &indices_to_prove,
        leaves_to_prove,
        leaves.len(),
    );
    println!("Proof verified: {}", verified);

    println!("--- End module: {}", module_path!());
}
