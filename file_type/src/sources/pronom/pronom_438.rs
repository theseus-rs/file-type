use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_438: FileType = FileType {
    file_format: &FileFormat {
        id: 438,
        source_type: SourceType::Pronom,
        name: "DEC WPS Plus Document",
        extensions: &["wpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
