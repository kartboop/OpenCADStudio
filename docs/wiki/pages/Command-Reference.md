# Command Reference

The lightweight edition registers only its current 2D commands. Use the command-line autocomplete to discover all available names in the running build.

The maintained summary in the source tree is `docs/commands-reference.md`; it covers file operations, drawing, editing, annotation, layers, blocks, references, measurement, and output.

Common commands include:

| Area | Commands |
| --- | --- |
| Files | `NEW`, `OPEN`, `QSAVE`, `SAVEAS`, `CLOSE` |
| Draw | `LINE`, `PLINE`, `CIRCLE`, `ARC`, `RECTANG`, `HATCH` |
| Modify | `MOVE`, `COPY`, `TRIM`, `EXTEND`, `OFFSET`, `FILLET`, `ERASE` |
| Annotation | `TEXT`, `MTEXT`, `DIMLINEAR`, `MLEADER`, `TABLE` |
| Organisation | `LAYER`, `BLOCK`, `INSERT`, `XREF`, `PROPERTIES` |
| View and inquiry | `ZOOM`, `PAN`, `DIST`, `AREA`, `MEASUREGEOM`, `LIST` |
| PDF and images | `PDFIMPORT`, `PDFATTACH`, `PDFSHXTEXT`, `IMAGEATTACH` |

Use `ALIASEDIT` to edit command aliases. Default aliases are defined in `assets/ocad.pgp`.
