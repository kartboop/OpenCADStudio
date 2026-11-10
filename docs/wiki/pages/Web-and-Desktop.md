# Web App and Desktop Program

Both targets share the lightweight 2D drawing core and support the same main DWG/DXF editing workflows. Neither includes 3D authoring, plug-ins, node graphs, external automation endpoints, or update checks.

| Area | Desktop | Web app |
| --- | --- | --- |
| Opening | Native file dialog, drag and drop, recent files, direct paths | Browser file picker or drag and drop |
| Saving | Selected filesystem path, local backups and autosave | Browser download or an available file-system handle |
| Windows | Native application window and in-app dialogs | One browser page with in-canvas dialogs |
| Fonts | Stroke fonts and installed TrueType fonts | Embedded stroke fonts and bundled web fonts |
| PDF and images | Supported import, underlay, and output workflows | Supported by the shared core and browser APIs |
| Printing | System printing where supported, and PDF | PDF/browser-oriented output |
| Graphics | Native wgpu backend | WebGL2 |
| Processing | Native worker threads | Main thread plus the DWG/DXF Web Worker |
| Native file associations and thumbnails | Supported | Not available |
| Startup `--script` option | Supported | Not available; use `SCRIPT` in the app |

Desktop builds generally handle large drawings faster because they can use multiple processor cores. Web parsing and serialization use a worker to keep the page responsive.

Browser settings and session data belong to the local site origin. Do not clear site data if you need to retain them.

For build commands, see [Build and Start](Installation) or the source tree's main README.
