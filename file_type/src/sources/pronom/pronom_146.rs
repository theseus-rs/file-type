use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_146: FileType = FileType {
    file_format: &FileFormat {
        id: 146,
        source_type: SourceType::Pronom,
        name: "AutoCAD ACIS Export File",
        extensions: &["sat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
