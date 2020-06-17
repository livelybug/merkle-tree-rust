// use std::error::Error;
use sha2::{Sha256, Digest};
use std::str;

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

type Bit256 = String;

fn get_sha256(input: impl AsRef<[u8]> ) -> Bit256{
    let mut hasher = Sha256::new();
    hasher.update(input);
    hex::encode(hasher.finalize().as_slice())  // ? sha2 crate have method to convert result to hex string?
}

pub fn create_merkle_tree(txs: Vec<&str>) {
    // Compute the hashes of input tx
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

}
