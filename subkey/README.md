FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    generate             Generate a random account
    generate-node-key    Generate a random node libp2p key, save it to file or print it to stdout and print its peer
                         ID to stderr
    help                 Prints this message or the help of the given subcommand(s)
    insert               Insert a key to the keystore of a node
    inspect              Gets a public key and a SS58 address from the provided Secret URI
    inspect-node-key     Print the peer ID corresponding to the node key in the given file
    module-id            Inspect a module ID address
    sign                 Sign a message, with a given (secret) key
    vanity               Generate a seed that provides a vanity address
    verify               Verify a signature for a message, provided on STDIN, with a given (public or secret) key