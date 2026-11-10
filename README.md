# Open CAD Studio — lightweight 2D edition

A local, simplified CAD workspace for viewing and editing engineering drawings. The application runs on desktop and in a browser, with no required account or cloud conversion service, analytics, sponsorship, or plugin runtime.

[简体中文说明](docs/readme/README.zh-CN.md)

## Included

- DWG, DXF, DWT, drawing backup, and autosave workflows
- PDF and raster-image viewing or underlays
- Lines, polylines, arcs, circles, splines, regions, and hatches
- Text, SHX fonts, dimensions, leaders, tables, and annotation scales
- Layers, blocks, external references, model space, and paper space
- Zoom, pan, measurement, object snaps, properties, and basic editing
- SVG import and PDF/CSV output where supported by the platform

This edition removes 3D authoring and peripheral 3D-model formats, the node graph, native plugins, external automation endpoints, update checks, and promotional or social features.

It does **not** add OCR, AI-based drawing recognition, or automatic quantity takeoff. Existing CAD text and SHX geometry can be displayed and edited; scanned drawings still require a separate recognition workflow.

## Run the web app

Install a current Rust toolchain, the WebAssembly target, Trunk, and `wasm-bindgen-cli`, then run:

```bash
rustup target add wasm32-unknown-unknown
trunk serve --release --offline --address 127.0.0.1 --port 8080
```

Open <http://127.0.0.1:8080/app/>. Drawing files stay in the browser unless you explicitly download or save them.

Build static deployment files with:

```bash
trunk build --release
```

The output is written to `dist/app/`.

## Run the desktop app

```bash
cargo run --bin OpenCADStudio
```

For an optimized executable:

```bash
cargo build --release --bin OpenCADStudio
```

## Verify

```bash
cargo check --workspace
cargo check --target wasm32-unknown-unknown -p OpenCADStudio -p ocs_web_worker
cargo test --workspace
```

Keep backups of important drawings. This remains an actively developed CAD codebase, and complex third-party files should be validated before production use.

## License

Open CAD Studio is distributed under the [GNU General Public License v3.0](LICENSE).
