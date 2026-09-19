# Setup

This documents how to set up a build environment, flash the device once with its BLE stack, and build and run the firmware.

## Prerequisites

For Linux distribution only. The Rust toolchain is required to build the firmware, and the probe-rs tools to flash and run it on the device.

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

## Set Up

These one-time steps enable defmt's binary analysis (`llvm-tools`) and install the target for the board's nRF52833 Cortex-M4F (`thumbv7em-none-eabihf`).

```shell
$ rustup component add llvm-tools
```

```shell
$ rustup target add thumbv7em-none-eabihf
```

## Flash BLE Stack (SoftDevice)

The BLE radio is provided by a Nordic SoftDevice, a binary that must live on the chip before the firmware runs. It only needs to be flashed once per device.

Download `s140_nrf52_7.3.0_softdevice.hex`. The official source is
https://www.nordicsemi.com/Products/Development-software/s140/download (free login). It is also mirrored on GitHub (and the Nordic site is behind a Cloudflare login wall, so the mirror is easier to script):

```shell
$ curl -sSLo s140_nrf52_7.3.0_softdevice.hex \
    https://raw.githubusercontent.com/adafruit/Adafruit_nRF52_Bootloader/master/lib/softdevice/s140_nrf52_7.3.0/s140_nrf52_7.3.0_softdevice.hex
```

Verify the download is a valid Intel HEX file (first line starts with `:020000040000FA`):

```shell
$ head -1 s140_nrf52_7.3.0_softdevice.hex
```

A full erase is mandatory before flashing the SoftDevice: erase the whole chip first, then flash the hex:

```shell
$ probe-rs erase --chip nRF52833_xxAA --allow-erase-all
$ probe-rs download --verify --binary-format hex --chip nRF52833_xxAA s140_nrf52_7.3.0_softdevice.hex
```

Check the SoftDevice is present: the first flash words must be a valid stack pointer and the MBR reset vector (e.g. `20000400` and `00000a81`), not all `0xFFFFFFFF`:

```shell
$ probe-rs read --chip nRF52833_xxAA b32 0x00000000 4
```

After flashing the SoftDevice for the first time, power-cycle or reset the board once before running the firmware: the very first boot initializes the MBR, which can fault once at `0x00000a80` (MBR entry). Once the board has booted once, every subsequent build only needs `cargo run`.

## Build

Compiles the firmware in release mode. Build output is expected to finish with a `Finished` line and no errors; defmt logs and symbols require the release profile's debug info.

```shell
$ cargo build --release
```

## Clean

Removes all build artifacts. Use it before rebuilding from scratch, e.g. to pick up toolchain or dependency changes.

```shell
$ cargo clean
```

## Run

Flashes the firmware onto the device and runs it via the `probe-rs run` runner configured in `.cargo/config.toml`, streaming defmt log output over RTT. Requires the SoftDevice to already be flashed (see above) and a debug probe attached.

```shell
$ cargo run --release
```