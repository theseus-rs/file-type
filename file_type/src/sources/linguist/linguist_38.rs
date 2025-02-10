use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_38: FileType = FileType {
    file_format: &FileFormat {
        id: 38,
        source_type: SourceType::Linguist,
        name: "Brainfuck",
        extensions: &["b", "bf"],
        media_types: &["text/x-brainfuck"],
        signatures: &[],
        related_formats: &[],
    },
};
