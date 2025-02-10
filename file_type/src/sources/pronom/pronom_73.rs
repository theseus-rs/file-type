use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_73: FileType = FileType {
    file_format: &FileFormat {
        id: 73,
        source_type: SourceType::Pronom,
        name: "Wordperfect Secondary File",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
