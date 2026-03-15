# Embedded Rust Framework

## Overview
To do

## Prerequisites
[Rust installation](https://rust-lang.org/fr/learn/get-started/)
[Qemu installation](https://www.qemu.org/download/)

## Set Up
```shell
$ curl -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```
```shell
$ rustup component add llvm-tools
```
```shell
$ rustup target add thumbv7em-none-eabihf
```

## Build
```shell
$ cargo build
```

## Clean
```shell
$ cargo clean
```

## Run
```shell
$ probe-rs list
```

```shell
$ cargo run
```

```shell
$ cargo debug
```