// View module — viewport tools, navigation, visual styles, palettes, interface.

mod cascade;
mod count_palette;
pub mod dview;
mod file_tabs;
mod graphic_attributes_palette;
mod layout_tabs;
pub mod limits;
pub(crate) mod ortho;
mod pan;
pub mod plot_window;
mod properties_palette;
pub mod quick_print;
mod sheetset;
mod tile_horiz;
mod tile_vert;
mod tool_palettes;
pub mod ucs_cmd;
pub mod view_cmd;
pub(crate) mod view_top;
pub mod visual_style;
pub(crate) mod vports_config;
mod vports_highlight;
mod vports_join;
mod vports_named;
mod vports_restore;
pub(crate) mod zoom_ext;
pub(crate) mod zoom_in;
pub(crate) mod zoom_out;
pub mod zoom_window;

use crate::modules::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

pub struct ViewModule;

impl CadModule for ViewModule {
    fn id(&self) -> &'static str {
        "view"
    }
    fn title(&self) -> &'static str {
        "View"
    }

    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            vec![
                RibbonGroup {
                    title: "Navigate",
                    tools: vec![
                        RibbonItem::LargeTool(zoom_ext::tool()),
                        RibbonItem::Tool(zoom_window::tool()),
                        RibbonItem::Tool(zoom_in::tool()),
                        RibbonItem::Tool(zoom_out::tool()),
                        RibbonItem::Tool(pan::tool()),
                    ],
                },
                RibbonGroup {
                    title: "Drawing Viewports",
                    tools: vec![
                        vports_config::dropdown(),
                        RibbonItem::Tool(vports_named::tool()),
                        RibbonItem::Tool(vports_join::tool()),
                        RibbonItem::Tool(vports_restore::tool()),
                        RibbonItem::Tool(vports_highlight::tool()),
                    ],
                },
                // ── Palettes ──────────────────────────────────────────────────────
                RibbonGroup {
                    title: "Palettes",
                    tools: vec![
                        RibbonItem::LargeTool(tool_palettes::tool()),
                        RibbonItem::LargeTool(properties_palette::tool()),
                        RibbonItem::LargeTool(count_palette::tool()),
                        RibbonItem::LargeTool(graphic_attributes_palette::tool()),
                        RibbonItem::LargeTool(sheetset::tool()),
                    ],
                },
                // ── Interface ─────────────────────────────────────────────────────
                RibbonGroup {
                    title: "Interface",
                    tools: vec![
                        RibbonItem::LargeTool(file_tabs::tool()),
                        RibbonItem::LargeTool(layout_tabs::tool()),
                        RibbonItem::Tool(tile_horiz::tool()),
                        RibbonItem::Tool(tile_vert::tool()),
                        RibbonItem::Tool(cascade::tool()),
                    ],
                },
                // ── Plot ──────────────────────────────────────────────────────────
                // Model space has no paper-space side toolbar, so Page Setup
                // (format/orientation/pick window for PLOTWINDOW) needs an
                // entry here too.
                RibbonGroup {
                    title: "Plot",
                    tools: vec![RibbonItem::Tool(ToolDef {
                        id: "PAGESETUP",
                        label: "Page Setup",
                        icon: IconKind::Svg(include_bytes!("../../../assets/icons/pagesetup.svg")),
                        event: ModuleEvent::Command("PAGESETUP".to_string()),
                    })],
                },
            ]
        })
    }
}
