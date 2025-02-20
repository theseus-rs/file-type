use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_547: FileType = FileType {
    file_format: &FileFormat {
        id: 547,
        source_type: SourceType::Pronom,
        name: "XYWrite for Windows Document",
        extensions: &["xyw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
