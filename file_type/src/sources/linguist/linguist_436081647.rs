use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_436081647: FileType = FileType {
    file_format: &FileFormat {
        id: 436_081_647,
        source_type: SourceType::Linguist,
        name: "Tree-sitter Query",
        extensions: &["scm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
