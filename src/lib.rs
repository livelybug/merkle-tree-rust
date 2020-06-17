//! # merkle_tree_rust
//! `merkle_tree_rust` contains methods to generate string list and the
//! merkle tree upon the list
use sha2::{Sha256, Digest};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


/// Parse the 2nd element of an list, then convert it to a integer variable
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

/// Generate a string list whose size is ```tx_num```
///
/// # Examples
///
/// ```
/// let txs = merkle_tree_rust::generate_txs(99);
///  assert_eq!(99, txs.len());
/// ```
pub fn generate_txs(tx_num: usize) -> Vec<String> {
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

/// Create a merkle tree from a string list by the following steps:
/// * Hash each element of the string list, make a new list from the hashing results.
/// * Take the new list as level 0 of the merkle tree
/// * Iterate each 2 elements of the top level of the merkle tree, calculate hashing results from each 2 elements, push the hashing results as top level of the merkle tree
/// * If the top level of the merkle tree has more than one elements, repeat last step. Otherwise, return the merkle tree.
///
/// # Examples
///
/// ```
/// let mut txs = Vec::new();
/// txs.push("a".into()); txs.push("b".into()); txs.push("c".into());
/// txs.push("d".into());
/// let merkle_tree = merkle_tree_rust::create_merkle_tree(&txs).unwrap();
/// assert_eq!("58c89d709329eb37285837b042ab6ff72c7c8f74de0446b091b6a0131c102cfd", merkle_tree.last().unwrap()[0]);
/// ```
///
/// # Errors
/// An error is returned if the string list input is empty
pub fn create_merkle_tree(txs: &Vec<String>) -> Result<Vec<Vec<Bit256>>, String>{
// ??&str has better compabilities/performance than String, but difficult to use Vec<&str> here??
    // Cannot accept empty tx list
    if txs.len() == 0 {
        return Err("String list input cannot be empty!".into());
    }

    // Compute the hashes of input txs
    let mut tx_hashes= Vec::new();
    for tx in txs {
        tx_hashes.push(get_sha256(tx));
    }

    // The hashes of input tx as level 0 of merkel tree
    let mut merkle_tree = Vec::new();
    merkle_tree.push(tx_hashes);

    // if the last level has more than 1 leaf, continue calculating
    while merkle_tree.last().unwrap().len() > 1 {
        let hashes = merkle_tree.last().unwrap();
        // println!("current level = {}, hashes = {:?}", merkle_tree.len() - 1, hashes);
        let mut hashes_computed = Vec::new();
        for idx in (0..hashes.len()).step_by(2) {
            if idx + 1 < hashes.len() { // hashes[idx] is not the last element of an odd sized list
                let combined_str = format!("{}{}", hashes[idx], hashes[idx + 1]);
                let res = get_sha256(&combined_str);
                hashes_computed.push(res);
            } else {  // hashes[idx] is the last element of an odd sized list
                let combined_str = hashes[idx].repeat(2); // duplicate the last leaf
                let res = get_sha256(&combined_str);
                hashes_computed.push(res);
            }
        }
        merkle_tree.push(hashes_computed);
    }
    println!("last level = {}, root hash = {:?}", merkle_tree.len() - 1, merkle_tree.last().unwrap());
    Ok(merkle_tree)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parser() {
        let args = vec!["a".into(), "b".into()];
        let err = parse_args(&args).unwrap_err();
        assert_eq!(err, "invalid digit found in string, command example:\n\ncargo run 100");

        let args = vec!["a".into(), "0".into()];
        let err = parse_args(&args).unwrap_err();
        assert_eq!(err, "The argument must be an interger greater than 0!");

        let args = vec!["a".into()];
        let err = parse_args(&args).unwrap_err();
        assert_eq!(err, "At least one argument is expected, command example:\n\ncargo run 100");

        let args = vec!["a".into(), "100".into()];
        let tx_num = parse_args(&args).unwrap();
        assert_eq!(tx_num, 100);
    }

    #[test]
    fn test_txs_gen() {
        let txs = generate_txs(99);
        assert_eq!(99, txs.len());
    }

    #[test]
    fn test_merkle_tree() {
        let mut txs = Vec::new();
        let err = create_merkle_tree(&txs).unwrap_err();
        assert_eq!("String list input cannot be empty!", err);

        txs.push("a".into()); txs.push("b".into()); txs.push("c".into());
        txs.push("d".into());
        let merkle_tree = create_merkle_tree(&txs).unwrap();
        assert_eq!("58c89d709329eb37285837b042ab6ff72c7c8f74de0446b091b6a0131c102cfd", merkle_tree.last().unwrap()[0]);

        txs.push("e".into());
        let merkle_tree = create_merkle_tree(&txs).unwrap();
        assert_eq!("3615e586768e706351e326736e446554c49123d0e24c169d3ecf9b791a82636b", merkle_tree.last().unwrap()[0]);

        txs.push("f".into()); txs.push("g".into());
        let merkle_tree = create_merkle_tree(&txs).unwrap();
        assert_eq!("61198f165d0f10dc1cd3f688bb7c5cf9f0d6f892532a6ebd984fb9b6bb124dd8", merkle_tree.last().unwrap()[0]);
    }
}
