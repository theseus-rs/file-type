use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_430: FileType = FileType {
    file_format: &FileFormat {
        id: 430,
        source_type: SourceType::Linguist,
        name: "EBNF",
        extensions: &["ebnf"],
        media_types: &["text/x-ebnf"],
        signatures: &[],
        related_formats: &[],
    },
};
