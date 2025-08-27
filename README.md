<div align="center">

# `wuerfel`

**Diceware password generator cli based on eff password lists.**

[![Crates.io Version](https://img.shields.io/crates/v/wuerfel)](https://crates.io/crates/wuerfel)
[![lib.rs link](https://badgen.net/badge/lib.rs/lib.rs/purple?label)](https://lib.rs/crates/wuerfel)
[![GitHub License](https://badgen.net/github/license/WyvernIXTL/wuerfel-rs)](https://github.com/WyvernIXTL/wuerfel-rs/blob/master/LICENSE)
[![deps.rs](https://deps.rs/crate/wuerfel/latest/status.svg)](https://deps.rs/crate/wuerfel/)

</div>

[![asciicast](https://asciinema.org/a/OM6wZiuAYnRkPMLN0BltXBiig.svg)](https://asciinema.org/a/OM6wZiuAYnRkPMLN0BltXBiig)

## Installation

### Prebuilt

See the [release page](https://github.com/WyvernIXTL/wuerfel-rs/releases).

### Build from Source

Via [Cargo](https://www.rust-lang.org/tools/install):

```sh
cargo install wuerfel
```

## Usage

By default a password with at least 90 bits of entropy is generated:

```sh
wuerfel
# word count: 9
# entropy: 93.1 bits
# emu rerun film donor drab ride coat ruby grasp
```
To generate a stronger password you may use either the `--count` or the `--entropy` options:

```sh
wuerfel --count 12
wuerfel -c 12
# word count: 12
# entropy: 124.1 bits
# case walk tummy blink open shore thaw curl nutty tilt tall found
```
```sh
wuerfel --entropy 256
wuerfel -e 256
# word count: 25
# entropy: 258.5 bits
# self fried sled humid quilt fancy baker dad spend hers strut spoof shiny shirt stoop slush alarm brick sway plot lying cub acorn musky aroma
```

To copy the password to your clipboard, instead of printing it to the terminal, use the `--cb` flag:

```sh
wuerfel --cb
# word count: 9
# entropy: 93.1 bits
# clipboard is going to be deleted in 10s
# clipboard cleared!
```

Three wordlists are included:
* EFF Short Wordlist (default)
* EFF Long Wordlist
* EFF Short Memorable Wordlist 

```sh
wuerfel -l short
wuerfel -l long
wuerfel -l memorable
```
I explicitly recommend at least 110 bits of entropy for encryption purposes
and at least 80 bits of entropy for authentication purposes.

More is better!

## Security

* The password lists where unaltered taken from the eff webpage and are parsed on build time.
* The random values are generated stem from the ["operating-system’s random data source"](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html).
* The password and the random numbers are zeroized (but with not guarantee). 

