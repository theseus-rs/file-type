use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1289: FileType = FileType {
    file_format: &FileFormat {
        id: 1_289,
        source_type: SourceType::Pronom,
        name: "Bentley V8 DGN",
        extensions: &["dgn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
