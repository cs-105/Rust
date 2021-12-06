# Rust Asteroids Game

We have designed a new asteroids game through the programming language rust.

## Installation

Make sure to have Rust, cargo and sdl2 installed to run the application.

For installing Rust on a MacOS or Linux OS just simply run this command in terminal to install:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you are running a windows computer and need to install Rust use [this link](https://forge.rust-lang.org/infra/other-installation-methods.html#which) to find rustup-init.exe (this will install cargo as well for windows)

SDL2 installation varies across the different operating systems 
so follow [this link](https://www.libsdl.org/download-2.0.php) and navigate to your correct system for the appropriate file to download.

For Linux and MacOS users run this line of code to get cargo.

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Windows
Make sure you have Visual Studio installed (not Visual Studio Code), and download SDL2, [SDL2_image](https://www.libsdl.org/projects/SDL_image/), and [SDL2_ttf](https://www.libsdl.org/projects/SDL_ttf/). (Make sure to download the VC devel versions)

Extract the SDL2 archives, then add the .lib and .dll files to your Rust toolchain path.

For example:
```
C:\Users\%USER%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib
```

You should be good to continue to the next step.

## Running the Game

Simply start the programming by starting up cargo with the command...

```bash
cargo run
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
