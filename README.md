
# Kabegami

Kabegami is a simple and self-contained wallpaper setter written in Rust. It does not rely on any other wallpaper utilities, making it a minimal solution for setting your desktop background. The tool is designed to be customizable and work properly on all platforms and desktop environments.

## Supported Desktop Environments

Kabegami currently supports the following desktop environments:

-   Gnome
-   Kde
-   Xfce
-   Lxde
-   Lxqt
-   Mate
-   Cinnamon

In case your desktop environment is not supported, the wallpaper will be set using Xcb. however, multiple monitor support is currently not available with Xcb.
You can create a file named `setter.sh` in the config directory `$HOME/.config/kabegami` and add your custom setting command. The tool will run this script to set the background.

## Installation

You can install Kabegami using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), the package manager for Rust:
```
cargo install kabegami
```

## Usage

```
Usage: kabegami [<PATH>] [--mode <mode>] [-V]

Simple Background Setter

Positional Arguments:
  PATH              the path to the images directory or image file

Options:
  --mode            default mode: strim, available modes: strim, stretch, fill
  -V, --version     print version info
  --help            display usage information
```

## Contributing

If you'd like to contribute to this project, feel free to submit a pull request or open an issue on the repository. All contributions are welcome!

## License

This project is licensed under the MIT License.
