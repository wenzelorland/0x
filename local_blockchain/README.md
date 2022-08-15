# Running a local geth node environment

## Install geth

1. To get started first install [geth](https://geth.ethereum.org/downloads/).
2. Check that installation was successful with `geth version`.

## Run a blockchain with two individual nodes

### Preparing clients and accounts

1. Make sure you are in the `local_blockchain` directory.
2. Create two accounts (one for each node) by running `geth --datadir node1 account new --password node1/password.txt` and `geth --datadir node2 account new --password node2/password.txt`.
3. Make sure to copy the two public keys that are shown in the CLI.
4. Initialize the two nodes with `geth init --datadir node1 genesis.json` and `geth init --datadir node2 genesis.json`. This uses the information in `genesis.json` to create the genesis block of your blockchain.

### Creating a Bootnode

The bootnode is used to help the clients to find each other to create a network.

1. Run `bootnode -genkey boot.key` to create a key for our bootnode.
2. Start the bootnode by running `bootnode -nodekey boot.key -addr :30305`.
3. Make sure to copy the `enode://<hash>@<host>=<port>` adress of the bootnode.

## Starting the clients

Make sure to run the command in new terminal tabs while the bootnode is running.

1. Simply run `geth --datadir node1 --port 30306 --bootnodes enode://<hash>@<host>=<port> --networkid 12345 --unlock <accounthash_for_node1> --password node1/password.txt --authrpc.port 8553` and `geth --datadir node2 --port 30307 --bootnodes enode://<hash>@<host>=<port> --networkid 12345 --unlock <accounthash_for_node2> --password node2/password.txt --authrpc.port 8554`
