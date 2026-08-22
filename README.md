# Embedded Rust Framework

## Overview
To do

## Prerequisites
For Linux distribution only
```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

## Set Up
```shell
$ cargo install probe-run
```
```shell
$ rustup component add llvm-tools
```
```shell
$ rustup target add thumbv7m-none-eabi
```

## Flash BLE Stack (SoftDevice)
Download `s140_nrf52_7.3.0_softdevice.hex` from
https://www.nordicsemi.com/Products/Development-software/s140/download (free login), then flash it once:
```shell
$ probe-rs erase --chip nRF52833_xxAA --allow-erase-all
$ probe-rs download --verify --binary-format hex --chip nRF52833_xxAA s140_nrf52_7.3.0_softdevice.hex
```
A full erase is mandatory before flashing the SoftDevice.

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
$ cargo run --release
```