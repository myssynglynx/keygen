# keygen
## Basic Deterministic & Non-Deterministic Keygen CLI Tool
### Installation:
Must have `cargo` installed. To install, clone repository, change directories into repository, and run
`cargo install --path .`.
```
git clone https://github.com/myssynglynx/keygen.git
cd keygen
cargo install --path .
```
### Usage:
```
keygen [OPTIONS]
```
###
| Tag                   | Description                                                                                            |
| --------------------- | -------------------------------------------------------------------------------------------------------|
| -l, --length <LENGTH> | Length of key to generate [default: 8]                                                                 |
| -f, --format <FORMAT> | Format of key to generate (uppercase, lowercase, numeric, alphanumeric, base64url) [default: numeric]  |
| -s, --seed <SEED>     | Generation seed [default: "hello world"]                                                               |
| -d, --deterministic   | Deterministically generate key [defaut: false]                                                         |
| -t, --takes           | Number of keys to generate [default: 1]                                                                |
| -h, --help            | Print help                                                                                             |
| -V, --version         | Print version                                                                                          |
---
### examples:
Non-deterministic key generation of 10 keys:
```
keygen -f base64url -t 10
```
```
Generating 10 random keys
WoD7Nv6TMkPsPoqSG2MPMz7NyYT9Rd8N-txxUybTZJnM2ZlArz68PPvhEJG2B06z
ypZGfjS9CErHpnNbqyBplKKTBf62-PvjVGNjrPF1-S5CVV0yZ-CEpNsLhVqWefrX
aDl_2h59uxboelwfZT4rUCMTaNkZSJm_r4v9snKYsbtsonXYsUt5pFIZXO_Pp5s_
GZNlfNUuO8PmkLbDEw2bI5G28rsBK56fBe0JUTY40H8Zn_J-wEJkeboMhxQ9wskW
7W9x7ywbyl5W1uPHn7cXR5hy_9IgtiQyaBHbYu3jIzhDL3YgfFjAMUNFap9gBpt2
up43p10waTQOs-Rts2M59i16yrkmafLA_5zBLvj8uKSo4UB8_fNjsYtz4ZwJcG8J
m3Ti4sROH2Bvb_0n9QFb4UwEWkEvyqr_ityz_yelqNb81BSsTZyUUlV1EP8M1MV-
RcVy3_XddG6dK_bL1H3cBN3468cqkacB3w4JfqfAZchrIRfhPOaeA8QoOoW91ily
BAnHyJyYpFF0cb0Ydv28FriIDgaQGyVWUNTX8LVcqJaj7QIDUKSUaZfaOiWasTsy
zfSfzpWzpf4kEb7WG9kLrStAA6YAE9OVxo9Ydr4kNA2Ijyv3CXJlUE1opZw09Rij
```
---
Deterministic 8-digit key generation of 5 uppercase random keys:
```
keygen -f uppercase -s foobar -t 5 -d
```
```
Deterministically generating 5 random keys
CCQQMQZY
ARVBYNOV
QDBWJEHC
MTMZCQUI
ELHIJKMH
```
```
keygen -f uppercase -s foobar -t 5 -d
```
---
Deterministic 8-digit key generation of 5 lowercase random keys:
```
Deterministically generating 5 random keys
ccqqmqzy
arvbynov
qdbwjehc
mtmzcqui
elhijkmh
```
---
For questions, comments, or concerns, please email myssynglynx@heat.tech
