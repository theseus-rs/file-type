use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_758480799: FileType = FileType {
    file_format: &FileFormat {
        id: 758_480_799,
        source_type: SourceType::Linguist,
        name: "Lark",
        extensions: &["lark"],
        media_types: &["text/x-ebnf"],
        signatures: &[],
        related_formats: &[],
    },
};
