use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_61: FileType = FileType {
    file_format: &FileFormat {
        id: 61,
        source_type: SourceType::Pronom,
        name: "CorelDraw Template",
        extensions: &["cdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
