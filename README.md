# xrandr-brightness-gtk4-gui

A simple and modern GUI application for adjusting display brightness and gamma on Linux systems using xrandr and the Xorg graphics stack.

![imagen](https://github.com/user-attachments/assets/13cdec52-1370-48eb-aace-e70ffc26babf)

## Features

- **Intuitive interface**: Clean and easy-to-use GUI built with GTK4
- **Complete control**: Adjust brightness and gamma independently
- **Multi-monitor support**: Manage multiple connected displays
- **Optimal performance**: Developed in Rust for maximum efficiency
- **Native integration**: Uses xrandr directly without additional dependencies

## Requirements

- Linux operating system
- Xorg (X11) - not compatible with Wayland
- xrandr installed on the system
- GTK4

### System dependencies

#### Ubuntu/Debian
```bash
sudo apt install xrandr libgtk-4-dev
```

#### Fedora/RHEL
```bash
sudo dnf install xrandr gtk4-devel
```

#### Arch Linux
```bash
sudo pacman -S xorg-xrandr gtk4
```

## Installation

### From releases

1. Download the latest binary from [Releases](https://github.com/user/xrandr-brightness-gtk4-gui/releases)
2. Make it executable: `chmod +x xrandr-brightness-gtk4-gui`
3. Move it to your PATH: `sudo mv xrandr-brightness-gtk4-gui /usr/local/bin/`

### Build from source

#### Prerequisites
- Rust (version 1.70 or higher)
- Cargo

#### Steps
```bash
# Clone the repository
git clone https://github.com/user/xrandr-brightness-gtk4-gui.git
cd xrandr-brightness-gtk4-gui

# Build
cargo build --release

# Binary will be in target/release/
```

## Troubleshooting

### Application doesn't detect displays
```bash
# Verify that xrandr works
xrandr --listmonitors

# Verify you're using Xorg
echo $XDG_SESSION_TYPE
```

### Insufficient permissions
```bash
# Make sure your user can execute xrandr
xrandr --help
```

### Build issues
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
