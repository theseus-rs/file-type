use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2655: FileType = FileType {
    file_format: &FileFormat {
        id: 2_655,
        source_type: SourceType::Pronom,
        name: "B Source Code File",
        extensions: &["b"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
