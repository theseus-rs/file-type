use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
