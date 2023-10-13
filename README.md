# Chip-8 Emulator in Rust

Welcome to the Chip-8 emulator written in Rust! This emulator allows you to experience the magic of the classic Chip-8 system, a simple yet fascinating platform that dates back to the 1970s. The emulator is implemented in Rust, a systems programming language known for its performance, safety, and ease of use.

## Table of Contents

- [Requirements](#requirements)
  - [Rust](#rust)
  - [Linux](#linux)
  - [macOS](#macos)
  - [Windows (MSVC)](#windows-msvc)
  - [Windows (MinGW)](#windows-mingw)
- [Running](#running)
- [Description](#description)
- [Why Rust?](#why-rust)

## Requirements

Before running the Chip-8 emulator, ensure you have the necessary dependencies installed. Depending on your operating system, follow the instructions below to set up the prerequisites.

### Rust

You'll need Rust installed on your system. You can install Rust by following the official instructions at [https://www.rust-lang.org/](https://www.rust-lang.org/).


### Linux

You'll need the SDL2 library to build and run the emulator. Install it using your package management tool or through the SDL2 website:

- Ubuntu:

    ```bash
    sudo apt-get install libsdl2-dev
    ```

- Fedora:

    ```bash
    sudo dnf install SDL2-devel
    ```

- Arch (no separate regular and development packages):

    ```bash
    sudo pacman -S sdl2
    ```

You might also need a C compiler (gcc).

### macOS

On macOS, you can install SDL2 via Homebrew or MacPorts:

#### Homebrew

Install SDL2 using Homebrew:

```bash
brew install sdl2
```

#### MacPorts

Alternatively, use MacPorts to install SDL2:

```bash
sudo port install libsdl2
```

### Windows (MSVC)

For Windows users, follow these steps to install the necessary libraries:

1. Download MSVC development libraries from [SDL2 website](http://www.libsdl.org/). Look for the SDL2-devel-2.0.x-VC.zip file.

2. Unpack SDL2-devel-2.0.x-VC.zip to a folder of your choice.

3. Copy all lib files from `SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\` to the appropriate Rust library directory. Depending on your Rust version, it may be `C:\Program Files\Rust\lib\rustlib\x86_64-pc-windows-msvc\lib` or another folder you prefer.

4. Copy SDL2.dll from `SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\` into your Cargo project directory, next to your Cargo.toml file.

When distributing your game, ensure SDL2.dll is in the same directory as your compiled executable.

### Windows (MinGW)

For MinGW users, here's how to install the required libraries:

1. Download MinGW development libraries from [SDL2 website](http://www.libsdl.org/). Look for the SDL2-devel-2.0.x-mingw.tar.gz file.

2. Unpack the files to a folder of your choice.

3. Copy all lib files from `SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\lib` to the appropriate Rust library directory. Depending on your Rust version, it may be `C:\Program Files\Rust\lib\rustlib\x86_64-pc-windows-gnu\lib` or another folder you prefer.

4. Copy SDL2.dll from `SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\bin` into your Cargo project directory, next to your Cargo.toml file.

When distributing your game, ensure SDL2.dll is in the same directory as your compiled executable.

## Running

To run the Chip-8 emulator, follow these steps:

1. Build the emulator using Cargo. Open your terminal and navigate to the project directory containing the emulator's source code. Run the following command:

    ```bash
    cargo build
    ```

    This will compile the emulator, and the executable will be placed in the target directory.

2. Navigate to the output directory where the emulator executable is located.

3. Run the emulator by executing the `chip-8-emulator` binary, providing the path to your Chip-8 game or program as a command-line argument. For example:

    ```bash
    ./chip-8-emulator /path/to/your/game.ch8
    ```

    Replace `/path/to/your/game.ch8` with the actual path to the Chip-8 game or program you want to run.

Enjoy playing and exploring the world of Chip-8 games with this emulator!

## Description

The Chip-8 emulator is a fun and educational project that enables you to run vintage Chip-8 games and programs on modern hardware. Chip-8 was designed for early microcomputers, and its simplicity makes it an excellent starting point for emulator development.

This emulator in Rust faithfully recreates the Chip-8 environment, allowing you to load and play classic games like Pong, Space Invaders, and Tetris. It comes with a simple and intuitive interface for controlling the games.

Features of this emulator include:

- Accurate Chip-8 instruction set emulation.
- Graphics rendering.
- Sound support.
- Keyboard input.
- Game loading and execution.

Whether you're a fan of retro gaming, a student of computer history, or an aspiring emulator developer, this project is a fantastic opportunity to explore the inner workings of a vintage system and learn about Rust programming.

## Why Rust?

Writing the Chip-8 emulator in Rust offers several advantages:

1. **Performance**: Rust provides fine-grained control over system resources, making it an excellent choice for low-level emulation where performance is crucial.

2. **Safety**: Rust's memory safety features help prevent common programming errors, making the emulator more robust and reliable.

3. **Cross-Platform**: Rust's cross-platform support means that you can easily run this emulator on different operating systems without major modifications.

4. **Community Support**: The Rust community is active and supportive, making it a great environment for learning and collaboration.

By combining the simplicity of Chip-8 with the power of Rust, this emulator project presents an excellent opportunity to gain insights into both vintage computing and modern programming practices. Have fun exploring the world of Chip-8 games with this emulator!
