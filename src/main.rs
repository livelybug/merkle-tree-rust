use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let arg1 = merkle_tree_rust::parse_args(&args).unwrap_or(100);
    let tx_num = merkle_tree_rust::parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing args: {}" ,err);
        process::exit(1);
    });
    // Input - A list of transaction hashes
    let txs = vec!["a", "b", "c", "d", "e"];
    // Create a merkle tree from a list of hashes
    merkle_tree_rust::create_merkle_tree(txs);
}