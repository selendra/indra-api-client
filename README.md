# Selendra-client

### Introduction

Selendra-client is a client generic tools build to interact with indracore and others Substrate blockchain based.

### Feature

- [x] Set WSS Node service provider.
- [x] Generate Wallet.
- [x] Offline Wallet Storage.
- [x] List Wallet.
- [x] Check Balance.
- [x] Asset Transfer.
- [x] Restore and backup from Json file.

#### Install via Cargo

If you are a Rust developer, you can install via cargo:
```bash
$ cargo install operation --git https://github.com/selendra/selendra-client.git
```

#### Install from source

to Install the Selendra-Client from source

```sh
$ git clone https://github.com/selendra/selendra-client.git
$ cd selendra-client
$ ./script/init.sh
$ cargo build --release
```
* note
	- create .env file and input your rpc (default = ws://127.0.0.1:9944)
	- please install [Rust](https://www.rust-lang.org/tools/install) before build

### Usage

```bash
$ ./operation -h
operation -- A simple command line interface wallet for selendra/substrate.

Usage:
    operation <command> [<args>...]
    operation -h | --help
    operation --version
Commands:
    transfer           transfer token between account.
    balance            check amount of token.
    listaddresses      Prints the list of addresses and blance of each account.
    watchaddress       Add a watchonly address.
    getnewaddress      Generate a new address associated with label, deafult cryptography is sr25519
    restore            Restore address from json file
    backup             Backup specified address to local json file
Options:
    -h --help          Show this screen, or help about a command.
    -v --version       Show version.
See 'operation <command> --help' for information on a specific command.
```

### Subcommands

#### `getnewaddress`

Generate a new random address

```bash
$ ./operation getnewaddress --ed25519 --name "FIRST"
5E9CDTeZ2qE1tvio5KcQtYV3hjDGTyMLXdRmn9RuCYAhmTZc
```
Generate from secret phrase
```bash
$ ./operation getnewaddress --phrase "genius alone lottery oval jump name member journey since age dance arm" --name "genius"
5FvEDMPKxw9Yk8zak4G6sHbawV2Nyc8odeUKPo1m6U8RgZ3s
```

#### `listaddresses`

List all generated addresses

``` bash
$ ./operation listaddresses
Name            Address                                                 Crypto       Balance                  
FIRST           5DA3scF5FRbGioWoZ2gBNP12gx9UC2heT4fFu7cRwA5m1nBg        sr25519      100.0 unit                 
SECOND          5E9CDTeZ2qE1tvio5KcQtYV3hjDGTyMLXdRmn9RuCYAhmTZc        ed25519      40.78 unit                 
SHELL           5CZp9sb9sBYmbfhd7BJjMQ87RZ7xqrJoZELqYZaoSjDVMzyp        sr25519      8.236 unit 
```

#### `watchaddress`
Add watchonly address to local storage.
```bash
./operation watchaddress --addr "5GuoS4yGC4xQhatpZNRrx7hYxQxs6spZbExCX2618sEZRtg9" --name "skull"
5GuoS4yGC4xQhatpZNRrx7hYxQxs6spZbExCX2618sEZRtg9 is added
```
#### `backup` 

Backup address to local json file. The backed file can be restored on [https://testnet.selendra.org`](https://testnet.selendra.org/?rpc=wss%3A%2F%2Frpc-testnet.selendra.org).

``` bash
./operation backup --addr "5CZp9sb9sBYmbfhd7BJjMQ87RZ7xqrJoZELqYZaoSjDVMzyp" --file "~/SHELL.json" --password "123"
Address `5CZp9sb9sBYmbfhd7BJjMQ87RZ7xqrJoZELqYZaoSjDVMzyp` is backed up to file `~/SHELL.json`
```
#### `restore`

Restore address from json file.
``` bash
./operation restore --file "/SHELL.json" --password "123"
5CZp9sb9sBYmbfhd7BJjMQ87RZ7xqrJoZELqYZaoSjDVMzyp is restored
```
#### `getbalances`
Show the balances of addresses
```bash
./operation balance --free-balance "5HigHwALzgPyXbg9o6tJfHE49DDMKbZxgMATJuomY4mhCvFD"
balance 1.0 unit
```
#### `transfer`
Transfer balance 

Use label
```bash
./operation transfer --sender "SHELL" --receiver "5HigHwALzgPyXbg9o6tJfHE49DDMKbZxgMATJuomY4mhCvFD" --amount 100
>> Balance transfer extrinsic submitted: ()
        ** from: 5DA3scF5FRbGioWoZ2gBNP12gx9UC2heT4fFu7cRwA5m1nBg
        ** to: 5CZp9sb9sBYmbfhd7BJjMQ87RZ7xqrJoZELqYZaoSjDVMzyp
        ** amount 100.00 unit
```
Use address
```bash
./operation transfer --sender "5CZp9sb9sBYmbfhd7BJjMQ87RZ7xqrJoZELqYZaoSjDVMzyp" --receiver "5HigHwALzgPyXbg9o6tJfHE49DDMKbZxgMATJuomY4mhCvFD" --amount 100
>> Balance transfer extrinsic submitted: ()
        ** from: 5DA3scF5FRbGioWoZ2gBNP12gx9UC2heT4fFu7cRwA5m1nBg
        ** to: 5CZp9sb9sBYmbfhd7BJjMQ87RZ7xqrJoZELqYZaoSjDVMzyp
        ** amount 100.00 unit
```
How to use other tutorial find at [Selendra Official Document](https://docs.selendra.org)
