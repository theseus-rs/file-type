use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_527: FileType = FileType {
    file_format: &FileFormat {
        id: 527,
        source_type: SourceType::Pronom,
        name: "StatGraphics Data File",
        extensions: &["aws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
