use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2187: FileType = FileType {
    file_format: &FileFormat {
        id: 2_187,
        source_type: SourceType::Pronom,
        name: "Error File",
        extensions: &["err"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
