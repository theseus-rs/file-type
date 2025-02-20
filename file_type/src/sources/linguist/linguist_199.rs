use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_199: FileType = FileType {
    file_format: &FileFormat {
        id: 199,
        source_type: SourceType::Linguist,
        name: "Lex",
        extensions: &["l", "lex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
