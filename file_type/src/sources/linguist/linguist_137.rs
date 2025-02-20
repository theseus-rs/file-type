use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_137: FileType = FileType {
    file_format: &FileFormat {
        id: 137,
        source_type: SourceType::Linguist,
        name: "Grammatical Framework",
        extensions: &["gf"],
        media_types: &["text/x-haskell"],
        signatures: &[],
        related_formats: &[],
    },
};
