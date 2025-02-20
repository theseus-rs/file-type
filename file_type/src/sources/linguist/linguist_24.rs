use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_24: FileType = FileType {
    file_format: &FileFormat {
        id: 24,
        source_type: SourceType::Linguist,
        name: "Assembly",
        extensions: &["a51", "asm", "i", "inc", "nas", "nasm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
