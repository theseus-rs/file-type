use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_114: FileType = FileType {
    file_format: &FileFormat {
        id: 114,
        source_type: SourceType::Pronom,
        name: "AutoCAD Template Menu File",
        extensions: &["mnu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
