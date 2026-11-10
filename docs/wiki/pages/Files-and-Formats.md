# Files and Formats

Open CAD Studio reads and writes DWG and DXF locally, without a conversion service.

## Opening and saving

Use **Ctrl+O** (`OPEN`), the application menu, drag and drop, or the recent drawings list. DWG, DXF, DWT, backup (`.bak`), and autosave (`.sv$`) files are supported.

| Command | Key | What it does |
| --- | --- | --- |
| `SAVE`, `QSAVE` | Ctrl+S | Save under the current name; the first save asks for one |
| `SAVEAS` | Ctrl+Shift+S | Save under a new name, format, or version |
| `SAVEALL` | | Save every open drawing that already has a path |

DWG and DXF can be saved in versions from R14 through 2018. Keep a backup and verify complex drawings after changing versions.

`ISAVEBAK` controls whether saving over a drawing keeps the previous file as `.bak`. `SAVETIME` controls the autosave interval; `SAVEFILEPATH` selects its folder. These are also available under **Options › Open and Save**.

## Templates

`QNEW` starts from the default `.dwt` template selected under **Options › Files**. `NEW` asks which template to use. Templates can contain layers, styles, layouts, and title blocks.

## Recovery and cleanup

| Command | What it does |
| --- | --- |
| `RECOVER` | Open a drawing in recovery mode |
| `RECOVERALL` | Recover a drawing and its references |
| `AUDIT` | Check drawing integrity |
| `PURGE` | Remove unused named definitions |
| `FINDNONPURGEABLE` | Explain why named items are still used |
| `OVERKILL` | Remove duplicate and overlapping geometry |
| `COMPARE` | Compare the open drawing with another one |

`DWGPROPS` shows drawing metadata and statistics. `ETRANSMIT` packages the drawing and its dependencies into a ZIP file.

## Import, attach, and export

| Format or workflow | Read/attach | Write/output |
| --- | --- | --- |
| DWG, DXF, DWT | Open | Save |
| BAK, SV$ | Open as recovery sources | Created by backup/autosave |
| PDF | `PDFIMPORT`, `PDFATTACH` | `PLOT`, `EXPORTPDF` |
| SVG, SVGZ | `SVGIN` | `EXPORTSVG` |
| Raster images | `IMAGEATTACH` | Available through plotting/output |
| DWF, DGN | Underlay workflows where supported | |
| LandXML | `LANDXMLIMPORT` for survey points | |
| CSV/XLSX | `DATALINK` and extraction workflows | `DATAEXTRACTION` |
| CTB, STB | Plot style tables | Plot style tables |

The former OBJ, 3DS, SAT, STEP, IGES, STL, FBX, and general model-import workflows are not included in this edition.

### IFCCAD (experimental)

**Save As › IFCCAD** writes the experimental IFCCAD profile of IFCX (`.ifccad`). Compatibility is not guaranteed between versions. Keep the original DWG or DXF and see the [IFCCAD guide](../../ifccad.md).

## Command scripts

Use `SCRIPT` to run a `.scr` file, or the desktop `--script` startup option. See [Command Scripts](Command-Scripts). External MCP, REST, and `--serve` interfaces have been removed.

## Web app

The browser selects local files and saves through downloads or an available browser file-system handle. There are no native file associations, system-printer integration, or startup command-line options in the browser.
