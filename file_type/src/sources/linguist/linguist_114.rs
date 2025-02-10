use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_114: FileType = FileType {
    file_format: &FileFormat {
        id: 114,
        source_type: SourceType::Linguist,
        name: "Forth",
        extensions: &["4th", "f", "for", "forth", "fr", "frt", "fs", "fth"],
        media_types: &["text/x-forth"],
        signatures: &[],
        related_formats: &[],
    },
};
