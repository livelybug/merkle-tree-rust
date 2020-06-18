//! # merkle_tree_rust
//! `merkle_tree_rust` contains methods to generate string list and the
//! merkle tree upon the list
use sha2::{Sha256, Digest};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use anyhow::Result;  // Redundant, but more readable
use anyhow::*;

/// Parse the 2nd element of the list input, then convert it to an integer.
///
/// Return an integer parsed.
///
/// # Examples
///
/// ```
/// let args = vec!["a".into(), "100".into()];
/// let tx_num = merkle_tree_rust::parse_args(&args).unwrap();
/// assert_eq!(tx_num, 100);
/// ```
///
/// # Errors
/// An error is returned under the following conditions:
/// * If the list input has less than 2 elements
/// * If the 2nd element of the list input is not an integer
/// * If the 2nd element of the list input is an integer, but its value is less than 1
/// * If the 2nd element of the list input is an integer, but greater than the OS' memory locating ability
pub fn parse_args(args: &[String]) -> Result<usize, String> {
    if args.len() < 2 {
        return Err("At least one argument is expected, command example:\n\ncargo run 100".into());
    }

    let tx_num = match args[1].parse::<usize>() {
        Ok(i) => {
            if i < 1 {
                return Err("The argument must be an interger greater than 0!".into());
            }
            else {
                i
            }
        },

        Err(e) => {
            let e_msg = format!("{}, command example:\n\ncargo run 100", e.to_string());
            return Err(e_msg);
        }
    };

    Ok(tx_num)
}

/// Generate a string list whose size is ```tx_num```.
///
/// Return a string list.
///
/// # Examples
///
/// ```
/// let txs = merkle_tree_rust::make_txs(99);
///  assert_eq!(99, txs.len());
/// ```
pub fn make_txs(tx_num: usize) -> Vec<String> {
    let mut txs = Vec::new();

    for _i in 0..tx_num {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .collect();
        txs.push(rand_string);
    }

    txs
}

type Bit256 = String;

fn get_sha256(input: impl AsRef<[u8]> ) -> Bit256{
    let mut hasher = Sha256::new();
    hasher.update(input);
    hex::encode(hasher.finalize().as_slice())  // ? sha2 crate have method to convert result to hex string?
}

// if the number of hashed is odd and greater than 1, duplicate the last hash
fn dupl_last_odd_list(mut hashes: Vec<Bit256>) -> Vec<Bit256> {
    if hashes.len() > 1 && hashes.len() % 2 == 1 {
        hashes.push(hashes.last().unwrap().into());
    }
    hashes
}

/// Create a merkle tree from a string list by the following steps:
/// * Hash each element of the string list, make a new list from the hashing results.
/// * Take the new list as level 0 of the merkle tree
/// * Iterate each 2 elements of the top level of the merkle tree, calculate hashing results from each 2 elements, push the hashing results as top level of the merkle tree
/// * If the top level of the merkle tree has more than one elements, repeat last step. Otherwise, return the merkle tree.
///
/// Return the merkle tree created.
///
/// # Examples
///
/// ```
/// let mut txs = Vec::new();
/// txs.push("a".into()); txs.push("b".into()); txs.push("c".into());
/// txs.push("d".into());
/// let merkle_tree = merkle_tree_rust::make_merkle_tree(&txs).unwrap();
/// assert_eq!("58c89d709329eb37285837b042ab6ff72c7c8f74de0446b091b6a0131c102cfd", merkle_tree.last().unwrap()[0]);
/// ```
///
/// # Errors
/// An error is returned if the string list input is empty
pub fn make_merkle_tree(txs: &Vec<String>) -> Result<Vec<Vec<Bit256>>>{
// ??&str has better compabilities/performance than String, but difficult to use Vec<&str> here??
    // Cannot accept empty tx list
    if txs.len() == 0 {
        return Err(anyhow!("Creating a merkle tree, string list input cannot be empty!"));
    }

    // Compute the hashes of input txs
    let mut tx_hashes= Vec::new();
    for tx in txs {
        tx_hashes.push(get_sha256(tx));
    }

    // The hashes of input tx as level 0 of merkel tree
    let mut merkle_tree = Vec::new();

    // if the number of hashed is odd and greater than 1, duplicate the last hash
    tx_hashes = dupl_last_odd_list(tx_hashes);

    merkle_tree.push(tx_hashes);

    // if the last level has more than 1 leaf, continue calculating
    while merkle_tree.last().unwrap().len() > 1 {
        let hashes = merkle_tree.last().unwrap();
        // println!("current level = {}, hashes = {:?}", merkle_tree.len() - 1, hashes);
        let mut hashes_computed = Vec::new();
        for idx in (0..hashes.len()).step_by(2) {
            let combined_str = format!("{}{}", hashes[idx], hashes[idx + 1]);
            let res = get_sha256(&combined_str);
            hashes_computed.push(res);
        }

        // if the number of hashed is odd and greater than 1, duplicate the last hash
        hashes_computed = dupl_last_odd_list(hashes_computed);

        merkle_tree.push(hashes_computed);
    }

    println!("last level = {}, root hash = {:?}", merkle_tree.len() - 1, merkle_tree.last().unwrap());
    Ok(merkle_tree)
}

#[derive(Debug)]
pub struct ProofElement(i8, Bit256);

fn _get_merkle_proof(merkle_tree: &Vec<Vec<String>>, hash: &Bit256, mut proof: Vec<ProofElement>, mut level: usize) -> Vec<ProofElement> {
    if merkle_tree[level].len() == 1 {
        return proof;
    }

    // Get the index of current hash from the current level of merkle tree
    let index = merkle_tree[level].iter().position(|r| r.eq(hash) ).unwrap();

    let tmp_idx = (index % 2) as i8;

    // If ```temp_idx``` is 1, the proof element tuple is ```(0, merkle_tree[level][index - 1].clone())```
    // If ```temp_idx``` is 0, the proof element tuple is ```(1, merkle_tree[level][index + 1].clone())```
    let paired_hash = merkle_tree[level][index + 1 - (tmp_idx * 2) as usize].clone();
    proof.push(ProofElement((tmp_idx - 1) * -1, paired_hash));

    // Get the hash linked to current hash and paired hash
    let _hash = merkle_tree[level + 1][(index - tmp_idx as usize) / 2].clone();

    if level + 1 >= merkle_tree.len() {
        panic!("Something wrong when getting merkle proof, index out of boundary!")
    }
    level = level + 1;
    _get_merkle_proof(merkle_tree, &_hash, proof, level)
}

/// Create a merkle proof from a string list and one element of the list.
/// * The merkle proof is a tuple list. Each tuple is (pos, paired_hash), where pos is the position of the paired_hash in a hash pair. pos is 0 if the paired_hash is on the left of the pair.
///
/// Return the tuple list.
///
/// # Errors
/// An error is returned if the string list input is empty
pub fn get_merkle_proof(txs: &Vec<String>, tx: String) -> Result<Vec<ProofElement>> {
    // Cannot accept empty tx list
    if txs.len() == 0 {
        return Err(anyhow!("Creating a merkle proof, string list input cannot be empty!"));
    }

    // The transaction to be verified through the merkle proof must be existing in the original transaction list of the merkle tree
    if !txs.contains(&tx) {
        return Err(anyhow!("Creating a merkle proof of a transaction, but the transaction is not found in the transaction list!"));
    }

    // Get merkle tree
    let merkle_tree = make_merkle_tree(txs)?;

    // Create merkle proof - a tuple list
    let mut proof: Vec<ProofElement> = Vec::new();

    let hash = get_sha256(tx);
    proof = _get_merkle_proof(&merkle_tree, &hash, proof, 0);

    Ok(proof)
}

/// Calculate along the merkle proof input to get the merkle root.
///
/// Return the merkle root.
///
/// # Examples
///
/// ```
/// let mut txs = Vec::new();
/// txs.push("a".into()); txs.push("b".into()); txs.push("c".into());
/// txs.push("d".into());
/// let proof = merkle_tree_rust::get_merkle_proof(&txs, "a".into()).unwrap();
/// let root = merkle_tree_rust::get_root_by_proof("a".into(), proof).unwrap();
/// assert_eq!("58c89d709329eb37285837b042ab6ff72c7c8f74de0446b091b6a0131c102cfd" ,root);
/// ```
///
/// # Errors
/// An error is returned if the merkle proof input is empty.
pub fn get_root_by_proof(tx: &str, proof: Vec<ProofElement>) -> Result<Bit256> {
    // Cannot accept empty tx list
    if proof.len() == 0 {
        return Err(anyhow!("Calculate along the merkle proof to get the merkle root, the merkle proof cannot be empty!"));
    }

    // hash the tx
    let mut hash = get_sha256(tx);

    // verify through the proof
    for pe in proof {
        let concat = if pe.0 == 0 { format!("{}{}", pe.1, hash) } else { format!("{}{}",hash, pe.1) };
        hash = get_sha256(concat);
    }

    Ok(hash)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parser() {
        let args = vec!["a".into(), "b".into()];
        let err = parse_args(&args).unwrap_err();
        assert_eq!("invalid digit found in string, command example:\n\ncargo run 100", err);

        let args = vec!["a".into(), "0".into()];
        let err = parse_args(&args).unwrap_err();
        assert_eq!("The argument must be an interger greater than 0!", err);

        let args = vec!["a".into()];
        let err = parse_args(&args).unwrap_err();
        assert_eq!("At least one argument is expected, command example:\n\ncargo run 100", err);

        let args = vec!["a".into(), "100".into()];
        let tx_num = parse_args(&args).unwrap();
        assert_eq!(100, tx_num);
    }

    #[test]
    fn test_txs_gen() {
        let txs = make_txs(99);
        assert_eq!(99, txs.len());
    }

    #[test]
    fn test_merkle_tree() {
        let mut txs = Vec::new();
        let err = make_merkle_tree(&txs).unwrap_err();
        assert_eq!("Creating a merkle tree, string list input cannot be empty!", err.to_string());

        txs.push("a".into()); txs.push("b".into()); txs.push("c".into());
        txs.push("d".into());
        let merkle_tree = make_merkle_tree(&txs).unwrap();
        assert_eq!("58c89d709329eb37285837b042ab6ff72c7c8f74de0446b091b6a0131c102cfd", merkle_tree.last().unwrap()[0]);

        txs.push("e".into());
        let merkle_tree = make_merkle_tree(&txs).unwrap();
        assert_eq!("3615e586768e706351e326736e446554c49123d0e24c169d3ecf9b791a82636b", merkle_tree.last().unwrap()[0]);

        txs.push("f".into()); txs.push("g".into());
        let merkle_tree = make_merkle_tree(&txs).unwrap();
        assert_eq!("61198f165d0f10dc1cd3f688bb7c5cf9f0d6f892532a6ebd984fb9b6bb124dd8", merkle_tree.last().unwrap()[0]);
    }

    #[test]
    fn test_get_merkle_proof() {
        let mut txs = Vec::new();
        let err = get_merkle_proof(&txs, "a".into()).unwrap_err();
        assert_eq!("Creating a merkle proof, string list input cannot be empty!", err.to_string());

        txs.push("a".into()); txs.push("b".into()); txs.push("c".into());
        txs.push("d".into());
        let err = get_merkle_proof(&txs, "z".into()).unwrap_err();
        assert_eq!("Creating a merkle proof of a transaction, but the transaction is not found in the transaction list!", err.to_string());

        let proof = get_merkle_proof(&txs, "a".into()).unwrap();
        let root = get_root_by_proof("a", proof).unwrap();
        assert_eq!("58c89d709329eb37285837b042ab6ff72c7c8f74de0446b091b6a0131c102cfd" ,root);

        txs.push("e".into());
        let proof = get_merkle_proof(&txs, "e".into()).unwrap();
        let root = get_root_by_proof("e", proof).unwrap();
        assert_eq!("3615e586768e706351e326736e446554c49123d0e24c169d3ecf9b791a82636b", root);

        txs.push("f".into()); txs.push("g".into());
        let proof = get_merkle_proof(&txs, "f".into()).unwrap();
        let root = get_root_by_proof("f", proof).unwrap();
        assert_eq!("61198f165d0f10dc1cd3f688bb7c5cf9f0d6f892532a6ebd984fb9b6bb124dd8", root);
    }
}
