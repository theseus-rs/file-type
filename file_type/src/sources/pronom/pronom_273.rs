use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_273: FileType = FileType {
    file_format: &FileFormat {
        id: 273,
        source_type: SourceType::Pronom,
        name: "Turbo Debugger Keystroke Recording File",
        extensions: &["tdk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
