# Lightweight command reference

This is a focused reference for the current 2D edition. It does not list internal tokens or removed 3D, plug-in, and external-automation commands. The application's command-line autocomplete is the authoritative source for all registered names.

A leading `-` selects a command-line-oriented form where one exists. A leading apostrophe runs a supported display command transparently inside another command. Default aliases are stored in `assets/ocad.pgp` and can be edited with `ALIASEDIT`.

## Files and output

| Command | Purpose |
| --- | --- |
| `NEW`, `QNEW` | Create a drawing, optionally using the default template |
| `OPEN` | Open DWG, DXF, DWT, backup, or autosave files |
| `SAVE`, `QSAVE`, `SAVEAS`, `SAVEALL` | Save one or all drawings |
| `CLOSE`, `EXIT` | Close a drawing or leave the application |
| `RECOVER`, `RECOVERALL` | Open damaged drawings with recovery |
| `AUDIT` | Check drawing integrity |
| `PLOT`, `EXPORTPDF`, `EXPORTSVG` | Plot or export the current drawing |
| `PDFIMPORT` | Import supported PDF geometry as drawing objects |
| `PDFATTACH`, `IMAGEATTACH` | Attach a PDF or raster image |
| `SVGIN` | Import SVG/SVGZ geometry as a block |
| `ETRANSMIT` | Package a drawing and its dependencies |

## Draw and modify

| Command | Purpose |
| --- | --- |
| `LINE`, `PLINE`, `XLINE` | Draw lines, polylines, or construction lines |
| `CIRCLE`, `ARC`, `ELLIPSE` | Draw circles, arcs, or ellipses |
| `RECTANG`, `POLYGON`, `SPLINE` | Draw rectangles, polygons, or splines |
| `HATCH`, `GRADIENT`, `BOUNDARY` | Create fills and boundaries |
| `MOVE`, `COPY`, `ROTATE`, `SCALE` | Transform selected objects |
| `ERASE`, `TRIM`, `EXTEND`, `BREAK` | Remove or cut geometry |
| `OFFSET`, `FILLET`, `CHAMFER` | Offset or join geometry |
| `STRETCH`, `MIRROR`, `ALIGN` | Adjust or mirror geometry |
| `PEDIT`, `SPLINEDIT`, `HATCHEDIT` | Edit existing geometry and fills |
| `UNDO`, `REDO` | Reverse or restore edits |

## Text and annotation

| Command | Purpose |
| --- | --- |
| `TEXT`, `MTEXT` | Create single-line or multiline text |
| `DDEDIT`, `TEXTEDIT` | Edit text |
| `DIMLINEAR`, `DIMALIGNED`, `DIMANGULAR` | Create linear or angular dimensions |
| `DIMRADIUS`, `DIMDIAMETER`, `DIMORDINATE` | Create radial, diameter, or ordinate dimensions |
| `MLEADER`, `QLEADER` | Create leaders |
| `TABLE`, `TABLESTYLE` | Create tables or edit table styles |
| `STYLE`, `DIMSTYLE`, `MLEADERSTYLE` | Manage annotation styles |
| `PDFSHXTEXT` | Recognize supported SHX text geometry in PDF input |
| `TXT2MTXT` | Combine text into multiline text |

## Drawing organization

| Command | Purpose |
| --- | --- |
| `LAYER`, `LAYERS` | Open layer tools |
| `BLOCK`, `WBLOCK` | Define or write blocks |
| `INSERT`, `BEDIT` | Insert or edit a block |
| `XREF`, `XATTACH`, `XCLIP` | Manage and clip external references |
| `PROPERTIES` | Open the Properties panel |
| `QSELECT`, `FILTER` | Select objects by properties |
| `PURGE`, `OVERKILL` | Remove unused definitions or duplicate geometry |

## View and inquiry

| Command | Purpose |
| --- | --- |
| `ZOOM`, `ZOOM E` | Zoom or fit all geometry |
| `PAN` | Pan the view |
| `DIST`, `AREA`, `MEASUREGEOM` | Measure distance, area, or geometry |
| `LIST` | Inspect selected entity data |
| `ID` | Report a picked coordinate |
| `REGEN`, `REDRAW` | Refresh the display |
| `OSNAP`, `DSETTINGS` | Configure snaps and drafting aids |
| `SCRIPT` | Run a command script |
| `OPTIONS`, `ALIASEDIT`, `SHORTCUTS` | Configure the application and input |
