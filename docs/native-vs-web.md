# Desktop vs. Web

The lightweight edition uses the same 2D drawing core for native desktop and WebAssembly builds. Neither target includes 3D authoring, native plugins, external automation endpoints, or application update checks.

| Area | Desktop | Web |
| --- | --- | --- |
| Drawing support | DWG, DXF, DWT, backups, PDF and image workflows | Same core format support, subject to browser APIs |
| Open | Native picker, path, drag and drop, recent files | Browser picker or drag and drop; file bytes are parsed locally |
| Save | Write to a selected filesystem path | Browser download or an available browser file-system handle |
| Fonts | Stroke fonts, installed TrueType fonts, shaping and fallback | Embedded stroke fonts and bundled per-script web fonts |
| Graphics | wgpu backend provided by the operating system | WebGL2 |
| Parsing and save work | Native worker threads | Main thread plus the DWG/DXF Web Worker |
| Printing | System printing where supported, and PDF output | PDF/browser-oriented output |
| Native integration | File associations, thumbnails, single-instance handoff | None |
| Startup scripts | `--script` and the `SCRIPT` command | `SCRIPT` through the web file picker |
| External automation | Removed | Removed |

Large drawings generally open faster on desktop because native builds can use multiple processor cores. The browser worker keeps parsing and serialization off the page's main thread.

## Build and run

Desktop development build:

```bash
cargo run --bin OpenCADStudio
```

Desktop release build:

```bash
cargo build --release --bin OpenCADStudio
```

Web development server using the optimized build:

```bash
trunk serve --release --offline --address 127.0.0.1 --port 8080
```

Web release output:

```bash
trunk build --release
```

The static site is written to `dist/app/`. `Trunk.toml` sets `/app/` as the public URL and runs `scripts/build-web-worker.sh` after the main build.
