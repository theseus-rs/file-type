use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_83: FileType = FileType {
    file_format: &FileFormat {
        id: 83,
        source_type: SourceType::Pronom,
        name: "AutoCAD Drawing Template",
        extensions: &["dwt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
