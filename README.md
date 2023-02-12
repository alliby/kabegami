
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
Usage: kabegami [OPTIONS] <PATH>

Positional arguments:
  PATH                  the path to the images directory or image file

Options:
  -h, --help            display this help and exit
  -m, --mode MODE       default mode: strim, available modes: strim, stretch, fill (default: strim)

```

## Cross-Platform Support

Kabegami has been tested on Xfce and Bspw. Support for other platforms can be added as needed.

## Customization

Kabegami creates a config file in the path `$HOME/.config/kabegami` in the supported desktop environments. The file is named based on the current desktop environment and can be edited for customization.

## Contributing

If you'd like to contribute to this project, feel free to submit a pull request or open an issue on the repository. All contributions are welcome!

## License

This project is licensed under the MIT License.
