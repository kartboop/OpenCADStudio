# Build and Start

The lightweight edition can run as a desktop program or a browser application. Build it from the source tree with a current stable Rust toolchain.

## Web app

Install the WebAssembly target, Trunk, and the matching `wasm-bindgen-cli`:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

Start the optimized local app:

```bash
trunk serve --release --offline --address 127.0.0.1 --port 8080
```

Open <http://127.0.0.1:8080/app/>. Select drawings locally in the browser; save or download them only when you choose to do so.

Build static deployment files with:

```bash
trunk build --release
```

The output is `dist/app/`.

## Desktop

Run a development build:

```bash
cargo run --bin OpenCADStudio
```

Build an optimized executable:

```bash
cargo build --release --bin OpenCADStudio
```

The executable is `target/release/OpenCADStudio` (`OpenCADStudio.exe` on Windows). Linux builds also need the platform graphics and font development libraries listed in the [main README](../../../README.md).

To open a drawing directly:

```bash
cargo run --bin OpenCADStudio -- path/to/drawing.dwg
```

Use `--read-only` before the path to prevent saving.

## Settings

Settings, aliases, shortcuts, and custom support files are stored per user:

| System | Folder |
| --- | --- |
| Windows | `%APPDATA%\OpenCADStudio` |
| Linux | `~/.config/OpenCADStudio` or `$XDG_CONFIG_HOME/OpenCADStudio` |
| macOS | `~/Library/Application Support/OpenCADStudio` |

The application does not check for updates. Build a newer source revision when you want to upgrade, and keep backups of important drawings.
