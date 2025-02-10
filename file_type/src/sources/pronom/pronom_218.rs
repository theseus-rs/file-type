use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_218: FileType = FileType {
    file_format: &FileFormat {
        id: 218,
        source_type: SourceType::Pronom,
        name: "AutoCAD Film Roll",
        extensions: &["flm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
