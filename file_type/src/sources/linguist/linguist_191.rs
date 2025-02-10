use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_191: FileType = FileType {
    file_format: &FileFormat {
        id: 191,
        source_type: SourceType::Linguist,
        name: "LLVM",
        extensions: &["ll"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
