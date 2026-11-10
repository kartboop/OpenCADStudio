# System Variables

System variables control drawing and application behavior. Use `SETVAR` to inspect or change a supported variable, or use the corresponding Options/Drafting Settings page where one exists.

Common variables include:

| Area | Variables |
| --- | --- |
| Units and precision | `INSUNITS`, `LUNITS`, `LUPREC`, `AUNITS`, `AUPREC` |
| Files and recovery | `FILEDIA`, `ISAVEBAK`, `SAVETIME`, `SAVEFILEPATH` |
| Drafting | `ORTHOMODE`, `OSMODE`, `GRIDMODE`, `SNAPMODE` |
| Selection | `PICKADD`, `PICKDRAG`, `PICKBOX` |
| Annotation | `CANNOSCALE`, `ANNOAUTOSCALE`, `DIMASSOC` |
| Display | `FILETAB`, `TILEMODE`, `VPHIGHLIGHT` |
| References and images | `FRAME`, `IMAGEFRAME`, `PDFFRAME`, `XCLIPFRAME` |

The command line reports unknown or unavailable variables rather than silently changing them. Deleted 3D-authoring and plug-in variables are not part of the current command surface.
