# Layout RS

[![Build and Deploy](https://github.com/garutyunov/layout-rs/actions/workflows/deploy.yml/badge.svg)](https://github.com/garutyunov/layout-rs/actions/workflows/deploy.yml)

A keyboard layout editor and configurator built with Yew and Rust WebAssembly. This application allows you to create, edit, and manage custom keyboard layouts with an intuitive web interface.

## 🚀 [Try it live on GitHub Pages](https://garutyunov.github.io/layout-rs/)

## Features

- 📱 **Web-based Interface**: Edit keyboard layouts directly in your browser
- 💾 **Persistent Storage**: Save and load your custom layouts using local storage
- 🎹 **Visual Editor**: Interactive keyboard layout with clickable keys
- 🔄 **Layer Support**: Manage multiple keyboard layers
- 🎨 **Real-time Editing**: See changes immediately as you modify key mappings
- 🔄 **Import/Export**: Save your layouts and share them with others

## Getting Started

### Prerequisites

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Install [Trunk], the build tool for Rust-generated WebAssembly:

```bash
cargo install --locked trunk
```

### Development

Start the development server:

```bash
trunk serve
```

This will build the app, start a local server, and automatically rebuild when you make changes.

### Production Build

Build the app for production:

```bash
trunk build --release
```

The optimized build will be available in the `dist` directory.

## Deployment

This project is automatically deployed to GitHub Pages using GitHub Actions. The deployment workflow:

1. **Triggers**: Runs on every push to the `main` branch
2. **Build Process**: 
   - Installs Rust with `wasm32-unknown-unknown` target
   - Installs Trunk build tool
   - Builds the project with `trunk build --release`
   - Caches dependencies for faster builds
3. **Deploy**: Automatically deploys the built application to GitHub Pages

### Manual Deployment

To deploy manually to GitHub Pages:

1. Enable GitHub Pages in your repository settings
2. Set the source to "GitHub Actions"
3. Push to the `main` branch or manually trigger the workflow

The live application will be available at: `https://yourusername.github.io/layout-rs/`

## Usage

1. **Select a Key**: Click on any key in the keyboard layout to select it
2. **Edit Key Mapping**: Use the key editor to modify the selected key's function
3. **Switch Layers**: Use the layer controls to navigate between different keyboard layers
4. **Save Changes**: Click the save button to persist your layout changes
5. **Load Layouts**: Load previously saved layouts or start fresh

## Architecture

This project is built with:

- **[Yew](https://yew.rs/)**: A modern Rust framework for creating multi-threaded front-end web apps
- **[Trunk](https://trunkrs.dev/)**: A WASM web application bundler for Rust
- **WebAssembly**: For high-performance web applications
- **Web APIs**: Local storage for persistence, modern CSS for styling

## Components

- `Layout`: Main keyboard layout visualization
- `Keyboard`: Individual keyboard representation
- `Key`: Interactive key component with click handling
- `KeyEditor`: Interface for editing key mappings
- `Header`: Navigation and control interface
- `Keymap`: Data management for keyboard configurations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is dual licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

[trunk]: https://github.com/thedodd/trunk