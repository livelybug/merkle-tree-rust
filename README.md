# merkle-tree-rust
The small Rust programme create a merkle tree by the following:
* Accept one and only one user argument which is a number, as ```tx_num```
* Create a list of string as fake trasactions, which is the input of a merkle tree, the length of the list is ```tx_num```
* Create a merkle tree and store it in a 2D vector, which looks like
```
current level = 0, hashes = ["ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb", "3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d", "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6", "18ac3e7343f016890c510e93f935261169d9e3f565436429830faf0934f4f8e4", "3f79bb7b435b05321651daefd374cdc681dc06faa65e374e38337b88ca046dea"]
current level = 1, hashes = ["62af5c3cb8da3e4f25061e829ebeea5c7513c54949115b1acc225930a90154da", "d3a0f1c792ccf7f1708d5422696263e35755a86917ea76ef9242bd4a8cf4891a", "1a98a2105977d77929b907710dfad6b5f9cdae2abbcaa989a9387ed62c706cd1"]
current level = 2, hashes = ["58c89d709329eb37285837b042ab6ff72c7c8f74de0446b091b6a0131c102cfd", "463bb9d8f7fe77a1f4ea68498899ecec274cdf238783a42cb448ce1e2d8cbb6a"]
last level = 3, root hash = ["3615e586768e706351e326736e446554c49123d0e24c169d3ecf9b791a82636b"]
```

## Run the code
* Test
```commandline
cargo test
```

* Run
```bash
# The last argument indicates the number of transactions input, which is the input of a merkle tree
cargo run 99
cargo run 9999
```
