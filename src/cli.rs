use std::path::PathBuf;
use std::sync::OnceLock;

use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(
    name = "OpenCADStudio",
    version = env!("OCS_FULL_VERSION"),
    about = crate::t!("Open CAD Studio — 2D engineering drawings").into_owned(),
    long_about = None,
)]
pub struct Cli {
    #[arg(help = crate::t!("CAD files to open at startup (.dwg / .dxf).").into_owned(), long_help = None)]
    pub files: Vec<PathBuf>,
    #[arg(long)]
    pub new: bool,
    #[arg(long)]
    pub new_instance: bool,
    #[arg(long)]
    pub read_only: bool,
    #[arg(long, value_name = "BACKEND")]
    pub backend: Option<String>,
    #[arg(long, visible_alias = "no-gpu")]
    pub safe_mode: bool,
    #[arg(long)]
    pub compat_renderer: bool,
    #[arg(long, value_name = "FILE")]
    pub script: Option<PathBuf>,
    #[arg(long, value_name = "LEVEL")]
    pub log: Option<String>,
    #[arg(long, value_name = "BACKEND", hide = true)]
    pub gpu_probe: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct GuiConfig {
    pub files: Vec<PathBuf>,
    pub new: bool,
    pub read_only: bool,
    pub compat_renderer: bool,
    pub script_lines: Vec<String>,
    pub gpu_fallback_notice: Option<String>,
    pub gpu_compat_auto: bool,
}

pub static GUI_CONFIG: OnceLock<GuiConfig> = OnceLock::new();

pub fn gui_config() -> GuiConfig {
    GUI_CONFIG.get().cloned().unwrap_or_default()
}
