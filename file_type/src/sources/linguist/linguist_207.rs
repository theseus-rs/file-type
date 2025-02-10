use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_207: FileType = FileType {
    file_format: &FileFormat {
        id: 207,
        source_type: SourceType::Linguist,
        name: "Literate Haskell",
        extensions: &["lhs"],
        media_types: &["text/x-literate-haskell"],
        signatures: &[],
        related_formats: &[],
    },
};
