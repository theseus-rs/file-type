use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
