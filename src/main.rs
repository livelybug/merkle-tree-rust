use sha2::{Sha256, Digest};
use std::str;

type Bit256 = String;

fn get_sha256(input: impl AsRef<[u8]> ) -> Bit256{
    let mut hasher = Sha256::new();
    hasher.update(input);
    hex::encode(hasher.finalize().as_slice())  // ? sha2 crate have method to convert result to hex string?
}

fn create_merkle_tree(txs: Vec<&str>) {
    // Compute the hashes of original tx
    let mut tx_hashes= Vec::new();
    for tx in txs {
        tx_hashes.push(get_sha256(tx));
    }

    // The hashes of original tx as level 0 of merkel tree
    let mut merkle_tree = Vec::new();
    merkle_tree.push(tx_hashes);

    // if the last level has more than 1 leaf, continue calculating
    while merkle_tree.last().unwrap().len() > 1 {
        let hashes = merkle_tree.last().unwrap();
        println!("current level = {}, hashes = {:?}", merkle_tree.len() - 1, hashes);
        let mut hashes_computed = Vec::new();
        for idx in (0..hashes.len()).step_by(2) {
            if idx + 1 < hashes.len() { // not the last element of an odd sized list
                let combined_str = format!("{}{}", hashes[idx], hashes[idx + 1]);
                let res = get_sha256(&combined_str);
                hashes_computed.push(res);
            } else {  // the last element of an odd sized list
                // duplicate the last leaf
                let combined_str = hashes[idx].repeat(2);
                let res = get_sha256(&combined_str);
                hashes_computed.push(res);
            }
        }
        merkle_tree.push(hashes_computed);
    }
    println!("last level = {}, hashes = {:?}", merkle_tree.len() - 1, merkle_tree.last().unwrap());
}

fn main() {

    // Input - A list of transaction hashes
    let txs = vec!["a", "b", "c", "d", "e"];
    // Create a merkle tree from a list of hashes
    create_merkle_tree(txs);
}
