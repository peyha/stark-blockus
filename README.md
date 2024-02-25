
# Stark-blockus

Display Starknet's current block in real time.

## How to run

First, install [Rust](https://www.rust-lang.org/tools/install)

Then you can run the code with the following
`cargo run -- --rpc <STARKNET_RPC_URL>`

## Example 

```
╔═════════════════════════════════════════════════════════════════════════════════════════════════╗
║Block number: 584970                                                                             ║
║Timestamp: 1708868919                                                                            ║
║Block hash: 0x55e1983e1df179c590c0abd4fa8c87f832f62b23c4268f12bca5a636df7707d                    ║
║Parent hash: 0x4ca72ff773dda7dcc422ad14b07996a0d2eb60890c5c3b84bbc58d6c5986af                    ║
║Starknet version: 0.13.0                                                                         ║
║Block status: ACCEPTED_ON_L2                                                                     ║
║Sequencer address on mainnet is 0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8║
║New root is 0x21782bf35210ea501963bc356695be2bb7dd0219244578dc375eb3511effbb3                    ║
║L1 gas price is 29.97                                                                            ║
║Max fee: min=-2.08 gwei, max=1.94 gwei, avg=-0.20 gwei                                           ║
║Tx cnt: 54 , INVOKE: 54                                                                          ║
║Seen INVOKE type with version 1 51 times                                                         ║
║Seen INVOKE type with version 3 3 times                                                          ║
╚═════════════════════════════════════════════════════════════════════════════════════════════════╝
````

## Contribute

Feel free to raise any issue and/or a PR

## Improvements

This project can easily and infinitely be improved, few ideas:

- Improve tx decoding (according ot type)
- Start at a specific block