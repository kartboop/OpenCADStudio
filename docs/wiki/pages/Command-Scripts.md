# Command Scripts

A command script (`.scr`) is a plain-text sequence of the commands and answers you would enter in the CAD command line. It is useful for repeatable drafting and setup steps without an external automation service.

```text
; Draw a rectangular frame and fit it to the view
LINE 0,0 5000,0 5000,4000 0,4000 C
ZOOM E
```

Each non-empty line is submitted as one command line. Empty lines press Enter. Lines beginning with `;` or `#` are comments.

Run a script with `SCRIPT` or `SCR`. In the browser, the command opens a script file picker. On desktop, a script can also be supplied at startup:

```bash
OpenCADStudio --script setup.scr drawing.dwg
```

During development, the equivalent Cargo invocation is:

```bash
cargo run --bin OpenCADStudio -- --script setup.scr drawing.dwg
```

Use command-line forms such as `-LAYER`, `-INSERT`, or `-PLOT` when a script should answer prompts rather than open a dialog. Test scripts on a copy of the drawing first; command scripts can edit and save files.

The former MCP, REST, `--serve`, headless conversion, and native plug-in interfaces are not part of this lightweight edition.
