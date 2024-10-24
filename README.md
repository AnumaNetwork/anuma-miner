# Anuma-miner
[![Build status](https://github.com/AnumaNetwork/anuma-miner/workflows/ci/badge.svg)](https://github.com/AnumaNetwork/anuma-miner/actions)

[![dependency status](https://deps.rs/repo/github/AnumaNetwork/anuma-miner/status.svg)](https://deps.rs/repo/github/AnumaNetwork/anuma-miner)

A Rust binary for file encryption to multiple participants. 


## Installation
### From Sources
With Rust's package manager cargo, you can install anuma-miner via:

```sh
cargo install anuma-miner
```

### From Binaries
The [release page](https://github.com/AnumaNetwork/anuma-miner/releases) includes precompiled binaries for Linux, macOS and Windows.


# Usage
To start mining you need to run [anumad](https://github.com/AnumaNetwork/anumad) and have an address to send the rewards to.


Help:
```
anuma-miner 0.2.1
A Anuma high performance CPU miner

USAGE:
    anuma-miner [FLAGS] [OPTIONS] --mining-address <mining-address>

FLAGS:
    -d, --debug                   Enable debug logging level
    -h, --help                    Prints help information
        --mine-when-not-synced    Mine even when anumad says it is not synced, only useful when passing `--allow-submit-
                                  block-when-not-synced` to anumad  [default: false]
        --testnet                 Use testnet instead of mainnet [default: false]
    -V, --version                 Prints version information

OPTIONS:
        --devfund <devfund-address>            Mine a percentage of the blocks to the Anuma devfund [default: Off]
        --devfund-percent <devfund-percent>    The percentage of blocks to send to the devfund [default: 1]
    -s, --anumad-address <anumad-address>      The IP of the anumad instance [default: 127.0.0.1]
    -a, --mining-address <mining-address>      The Anuma address for the miner reward
    -t, --threads <num-threads>                Amount of miner threads to launch [default: number of logical cpus]
    -p, --port <port>                          Anumad port [default: Mainnet = 12413, Testnet = 12513]
```

To start mining you just need to run the following:

`./anuma-miner --mining-address anuma:XXXXX`

This will run the miner on all the available CPU cores.

# Devfund
**NOTE: This feature is off by default** <br>
The devfund is a fund managed by the Anuma community in order to fund Anuma development <br>
A miner that wants to mine a percentage into the dev-fund can pass the following flags: <br>
`anuma-miner --mining-address= XXX --devfund=anuma:qrg03ql6rnfvwd8a3z04j5a33l6f5d0ua8fkysftx8w7kacf0lsrqzxq2k2ud` <br>
and can pass `--devfund-precent=XX.YY` to mine only XX.YY% of the blocks into the devfund (passing `--devfund` without specifying a percent will default to 1%)

# Donation Address
`anuma:qrg03ql6rnfvwd8a3z04j5a33l6f5d0ua8fkysftx8w7kacf0lsrqzxq2k2ud`