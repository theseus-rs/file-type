use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_148: FileType = FileType {
    file_format: &FileFormat {
        id: 148,
        source_type: SourceType::Pronom,
        name: "AutoCAD Script",
        extensions: &["scr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
