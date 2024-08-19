# fri3d-rust-workshop
Programming Rust for ESP32 on the Fri3d Badge workshop

**WARNING:** this workshop is not for beginners. While knowledge of Rust is not required to get started and you can learn as you go, you should have experience programming for the ESP32 in another language and at least have some basic knowledge about compilers, toolchains and ESP-IDF.

# Prepare for the workshop

As it can take quite some time to get started, it is best if you setup the toolchain and try to compile and run the test program on the badge before the workshop starts. If not, you can easily spend the whole workshop on just the setup. If you experience any issues, try to find me ([Mathy Vanvoorden](https://github.com/MathyV)) during the camp in the CoderDojo Village or send me a message on Discord (username: mathyvanvoorden) or in the Fri3d Discord #badge channel.

First of all, choose on which system you want to develop. Note that no matter which platform you choose, you will need quite some free disk space (15GB+) for all the required toolchains and libraries so make sure you are set. If you go the WSL route, count an additional 10 GB just for the OS.

* **Linux native**: if you have a native Linux installation available, it will probably give you the best experience
  
  [Linux Setup Instructions](./docs/setup/linux)
* **WSL on Windows**: if you are running only Windows, you will get the best experience in a WSL container
  
  [WSL Setup Instructions](./docs/setup/wsl)
* **Windows native**: while it can work, my time is limited, feel free to create a PR with detailed instructions
* **MacOS with a VM**: you can run a Linux VM and follow the **Linux native** instructions
* **MacOS native**: Using [Homebrew](https://brew.sh/)  
  
  [MacOS Setup Instructions](./docs/setup/macos)

Follow the setup instructions for your platform of choice and come back here when ready.

## Building the code

### Clone this repository
```
git clone https://github.com/MathyV/fri3d-rust-workshop
cd fri3d-rust-workshop/example
```

### Building the example code
```
source ~/export-esp.sh
cargo build
```

### Flashing the image
```
cargo espflash flash
```

### To build and flash in 1 step
```
cargo run
```

You should see a `Hello, world!` message on the console and the lights on your badge should start flickering.
