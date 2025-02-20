use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_74: FileType = FileType {
    file_format: &FileFormat {
        id: 74,
        source_type: SourceType::Pronom,
        name: "Wordperfect Secondary File",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
