use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

const ROOT_HEX: &str = "1f7379539707bcaea00564168d1d4d626b09b73f8a2a365234c62d763f854da2";
const NEW_ROOT_HEX: &str = "e2a80e0e872a6c6eaed37b4c1f220e1935004805585b5f99617e48e9c8fe4034";

pub fn test() {
    println!("--- Start module: {}", module_path!());

    // Build Merkle tree.
    let leaf_values = ["a", "b", "c", "d", "e", "f"];
    let mut leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();

    // Create an empty merkle tree.
    let mut merkle_tree = MerkleTree::<Sha256>::new();
    assert_eq!(merkle_tree.leaves_len(), 0);

    // Append leaves.
    merkle_tree.append(&mut leaves);
    assert_eq!(merkle_tree.root_hex(), None);
    assert_eq!(
        merkle_tree.uncommitted_root_hex(),
        Some(ROOT_HEX.to_string())
    );
    println!(
        "Merkle tree root before commiting: {:?}",
        merkle_tree.root_hex().unwrap_or("None".to_string())
    );

    // Commit the changes.
    merkle_tree.commit();
    assert_eq!(merkle_tree.leaves_len(), leaf_values.len());
    assert_eq!(merkle_tree.root_hex(), Some(ROOT_HEX.to_string()));
    assert_eq!(merkle_tree.uncommitted_root_hex(), None);
    println!(
        "Merkle tree root after first commit: {:?}",
        merkle_tree.root_hex().unwrap()
    );

    // Add a new leaf and commit.
    merkle_tree.insert(Sha256::hash("g".as_bytes())).commit();
    assert_eq!(merkle_tree.leaves_len(), leaf_values.len() + 1);
    assert_eq!(merkle_tree.root_hex(), Some(NEW_ROOT_HEX.to_string()));
    println!(
        "Merkle tree root after second commit: {:?}",
        merkle_tree.root_hex().unwrap()
    );

    // Rollback.
    merkle_tree.rollback();
    assert_eq!(merkle_tree.leaves_len(), leaf_values.len());
    assert_eq!(merkle_tree.root_hex(), Some(ROOT_HEX.to_string()));
    println!(
        "Merkle tree root after first rollback: {:?}",
        merkle_tree.root_hex().unwrap()
    );

    // Rollback again.
    merkle_tree.rollback();
    assert_eq!(merkle_tree.leaves_len(), 0);
    assert_eq!(merkle_tree.root_hex(), None);
    println!(
        "Merkle tree root after first rollback: {:?}",
        merkle_tree.root_hex().unwrap_or("None".to_string())
    );

    println!("--- End module: {}", module_path!());
}
