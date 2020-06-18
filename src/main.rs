use std::env;
use std::process;

fn main() {
    // Parse args for the number which is the length of tx list
    let args: Vec<String> = env::args().collect();
    let tx_num = merkle_tree_rust::parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing args: {}" ,err);
        process::exit(1);
    });

    // Generate tx list of specified length
    let txs = merkle_tree_rust::make_txs(tx_num);

    // Create a merkle tree from a list of string as fake transctions
    let merkle_tree = merkle_tree_rust::make_merkle_tree(&txs).unwrap_or_else(|err| {
        eprintln!("Error creating merkle tree: {}" ,err);
        process::exit(1);
    });

    // Get merkle proof of a tx from a merkle tree
    let proof = merkle_tree_rust::get_merkle_proof(&txs, txs.last().unwrap().clone().as_str()).unwrap();
    // Verify the tx by merkle proof
    let root = merkle_tree_rust::get_root_by_proof(txs.last().unwrap().clone().as_ref(), &proof).unwrap();
    assert_eq!(root, merkle_tree.last().unwrap()[0]);
}