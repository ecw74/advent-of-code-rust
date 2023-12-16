# Advent of Code Example

## Setup

### Windows

1. Download [rustup‑init.exe](https://win.rustup.rs/x86_64) and install
2. Run `cargo install espup`
3. Run `cargo install cargo-generate`
4. Run `espup install`
5. Run `cargo install ldproxy`
6. Run `cargo install espflash`

### Linux

1. Install required packages:

   ```shell
   sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0 pkg-config
   ```
2. Install rustup `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Run `cargo install espup`
4. Run `cargo install cargo-generate`
5. Run `espup install`
6. Run `cargo install ldproxy`
7. Run `cargo install espflash`

## Create a new package

`cargo generate esp-rs/esp-idf-template cargo`

## Example

Create Cargo std project `cargo generate esp-rs/esp-idf-template cargo`

Create `cfg.toml`

```toml
[advent-of-code-rust]
wifi_ssid = "<ssid>"
wifi_psk = "<password>"
```

### WiFi Setup

https://dev.to/apollolabsbin/iot-with-rust-on-esp-connecting-wifi-4be6

### HTTP Server Setup

https://dev.to/apollolabsbin/edge-iot-with-rust-on-esp-http-server-5gf8

### Create New Package

```shell
cd common
cargo generate esp-rs/esp-idf-template cargo
cargo add embedded-svc esp-idf-svc anyhow
```