# Linux Native setup instructions

These instructions are written for Ubuntu, if you are running another distribution, you probably have enough knowledge on how to set it up on your platform of choice.

The documentation available online is pretty good so this serves more as a TL;DR. If you are serious about developing in Rust on microcontrollers, I suggest you go through the extensive available documentation.

# Update your system
It's best if you make sure your system is up-to-date before you start the installation.

```
sudo apt update
sudo apt upgrade
```

## Install Rust 
Follow instructions on https://rustup.rs/
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Select option 1 to proceed.

Exit your console and start it again to load the changed environment.

## Install esp-rs toolchain
Follow instructions on https://esp-rs.github.io/book/installation/riscv-and-xtensa.html
```
cargo install espup
espup install
```
Install additional tools
```
cargo install ldproxy espflash
cargo install --locked cargo-espflash
```
