use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
