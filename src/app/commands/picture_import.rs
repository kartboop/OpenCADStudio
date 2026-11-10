use super::{Message, OpenCADStudio};
use iced::Task;

async fn pick_svg() -> Option<String> {
    let handle = crate::sys::file_dialog()
        .set_title(crate::t!("Import SVG").as_ref())
        .add_filter("SVG (*.svg; *.svgz)", &["svg", "svgz"])
        .pick_file()
        .await?;
    let path = crate::sys::handle_path(&handle)
        .to_string_lossy()
        .into_owned();
    let bytes = handle.read().await;
    crate::scene::model::pdf_raster::register_source(&path, std::sync::Arc::new(bytes));
    Some(path)
}

impl OpenCADStudio {
    pub(super) fn dispatch_picture_import(
        &mut self,
        cmd: &str,
        i: usize,
    ) -> Option<Task<Message>> {
        let (verb, rest) = match cmd.trim().split_once(char::is_whitespace) {
            Some((verb, rest)) => (verb.to_ascii_uppercase(), rest.trim().trim_matches('"')),
            None => (cmd.trim().to_ascii_uppercase(), ""),
        };
        match (verb.as_str(), rest.is_empty()) {
            ("SVGIN", true) if crate::sys::file_dia() => Some(Task::perform(pick_svg(), |path| {
                match path {
                    Some(path) => Message::Command(format!("_SVGINFILE {path}")),
                    None => Message::Noop,
                }
            })),
            ("SVGIN", true) => {
                use crate::command::{CadCommand, FreeTextValuePromptCommand};
                let command = FreeTextValuePromptCommand::new(
                    "_SVGINFILE",
                    "Enter name of SVG file to import (no extension):",
                );
                self.command_line.push_info(&command.prompt());
                self.tabs[i].active_cmd = Some(Box::new(command));
                Some(Task::none())
            }
            ("SVGIN" | "_SVGINFILE", false) => {
                let path = if std::path::Path::new(rest).extension().is_some() {
                    rest.to_string()
                } else {
                    format!("{rest}.svg")
                };
                Some(self.import_svg(i, &path, true))
            }
            ("_SVGINFILE", true) => Some(Task::none()),
            _ => None,
        }
    }

    pub(in crate::app) fn import_svg(
        &mut self,
        i: usize,
        path: &str,
        prompt: bool,
    ) -> Task<Message> {
        let Some(bytes) = crate::scene::model::pdf_raster::source_bytes(path) else {
            self.command_line
                .push_error(&format!("File {path} does not exist."));
            return Task::none();
        };
        let (_, size, _) = self.tabs[i].scene.view_window();
        let picture = match crate::io::svg_import::read(&bytes, [0.5 * size[0], 0.5 * size[1]]) {
            Ok(picture) => picture,
            Err(crate::io::svg_import::Error::NotSvg) => {
                self.command_line
                    .push_error(&format!("{path} is not a valid SVG file."));
                return Task::none();
            }
            Err(crate::io::svg_import::Error::Empty) => {
                self.command_line
                    .push_error(&format!("{path} has no shapes to import."));
                return Task::none();
            }
        };
        self.push_undo_snapshot(i, if prompt { "SVGIN" } else { "IMPORT" });
        self.place_picture_block(i, "SVG", picture.entities, prompt)
    }

    pub(in crate::app) fn paste_svg_text(&mut self, i: usize, text: String) -> Task<Message> {
        let (_, size, _) = self.tabs[i].scene.view_window();
        match crate::io::svg_import::read(text.as_bytes(), [0.5 * size[0], 0.5 * size[1]]) {
            Ok(picture) => {
                self.push_undo_snapshot(i, "PASTECLIP");
                self.place_picture_block(i, "SVG", picture.entities, true)
            }
            Err(_) => {
                self.command_line
                    .push_error(crate::tr!("clipboard", "no-supported-content").as_ref());
                Task::none()
            }
        }
    }

    fn place_picture_block(
        &mut self,
        i: usize,
        prefix: &str,
        entities: Vec<document::EntityType>,
        prompt: bool,
    ) -> Task<Message> {
        let (centre, size, pixel) = self.tabs[i].scene.view_window();
        let scene = &mut self.tabs[i].scene;
        let mut n = 0;
        let name = loop {
            let name = format!("{prefix}{n}");
            if scene.document.block_records.get(&name).is_none() {
                break name;
            }
            n += 1;
        };
        if let Err(error) =
            scene.define_block_from_owned_entities(entities, &name, glam::DVec3::ZERO)
        {
            self.command_line.push_error(&error);
            return Task::none();
        }
        self.tabs[i].dirty = true;
        if prompt {
            use crate::command::CadCommand;
            use crate::modules::insert::insert_block::InsertBlockCommand;
            let preview = self.tabs[i].scene.block_preview_wires(&name);
            let command = InsertBlockCommand::classic_for_block(name, preview, String::new());
            self.command_line.push_info(&command.prompt());
            self.tabs[i].active_cmd = Some(Box::new(command));
        } else {
            let corner = document::types::Vector3::new(
                centre[0] - 0.5 * size[0] + pixel,
                centre[1] + 0.5 * size[1] - 2.0 * pixel,
                0.0,
            );
            let insert = document::entities::Insert::new(&name, corner);
            self.tabs[i]
                .scene
                .add_entity(document::EntityType::Insert(insert));
            self.refresh_properties();
        }
        Task::none()
    }
}
