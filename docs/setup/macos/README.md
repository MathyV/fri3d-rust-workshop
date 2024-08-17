# macOS Setup instructions

I used Homebrew to install Rust.

If you haven't already install Homebrew first:

```bash
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```


## Install Rust

```sh
  brew install rustup
```

Add `/usr/local/opt/rustup/bin:$HOME/.cargo/bin` to your PATH.

## Install esp-rs toolchain

Follow instructions on https://esp-rs.github.io/book/installation/riscv-and-xtensa.html

```sh
  cargo install espup
  espup install
```

## Install aditional tools

```sh
  cargo install ldproxy espflash
  cargo install --locked cargo-espflash
```
