#![allow(non_snake_case)]
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use OpenCADStudio::app;
#[cfg(target_arch = "wasm32")]
use OpenCADStudio::sys;
#[cfg(not(target_arch = "wasm32"))]
use OpenCADStudio::{cli, io};

fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        sys::web_diag::init();
        return app::run_web();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use clap::Parser;
        let args = cli::Cli::parse_from(std::env::args_os().map(|arg| {
            if arg == "/b" || arg == "/B" {
                "--script".into()
            } else {
                arg
            }
        }));
        if let Some(which) = &args.gpu_probe {
            OpenCADStudio::gpu_backend::run_probe_child(which);
        }
        OpenCADStudio::sys::crash_log::install();
        if let Some(level) = &args.log {
            unsafe { std::env::set_var("RUST_LOG", level) };
        }
        if std::env::var_os("RUST_LOG").is_some() {
            let _ = env_logger::try_init();
        }
        let gpu = OpenCADStudio::gpu_backend::resolve_gpu(args.backend.as_deref(), args.safe_mode);
        if !args.new_instance {
            if let io::single_instance::Claim::Existing(stream) = io::single_instance::claim() {
                let plain_open =
                    !args.read_only && args.script.is_none() && !args.new && !args.files.is_empty();
                if plain_open && io::single_instance::handoff(stream, &args.files) {
                    return Ok(());
                }
            }
        }
        let script_lines = args
            .script
            .as_ref()
            .map(|path| match std::fs::read_to_string(path) {
                Ok(text) => text
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.starts_with('#') && !line.starts_with(';'))
                    .map(str::to_string)
                    .collect(),
                Err(error) => {
                    eprintln!("--script: cannot read {}: {error}", path.display());
                    Vec::new()
                }
            })
            .unwrap_or_default();
        let gpu_fallback_notice = gpu
            .reason
            .map(|reason| OpenCADStudio::gpu_backend::fallback_notice(&reason));
        let saved_compat = OpenCADStudio::gpu_backend::load_prefs().compat_renderer;
        let compat_renderer = args.compat_renderer || saved_compat || gpu.compat_renderer;
        let gpu_compat_auto = !args.compat_renderer && !saved_compat && gpu.compat_renderer;
        let _ = cli::GUI_CONFIG.set(cli::GuiConfig {
            files: if args.new { Vec::new() } else { args.files },
            new: args.new,
            read_only: args.read_only,
            compat_renderer,
            script_lines,
            gpu_fallback_notice,
            gpu_compat_auto,
        });
        io::file_association::install_thumbnailer();
        if OpenCADStudio::gpu_backend::gpu_guard_active() {
            OpenCADStudio::gpu_backend::arm_sentinel(
                gpu.backend_value.as_deref().unwrap_or("auto"),
            );
            std::thread::spawn(|| {
                std::thread::sleep(OpenCADStudio::gpu_backend::SENTINEL_PROOF_DELAY);
                OpenCADStudio::gpu_backend::mark_sentinel_survived();
            });
            let result = app::run();
            OpenCADStudio::gpu_backend::disarm_sentinel();
            result
        } else {
            app::run()
        }
    }
}
