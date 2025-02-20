use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_247: FileType = FileType {
    file_format: &FileFormat {
        id: 247,
        source_type: SourceType::Linguist,
        name: "NewLisp",
        extensions: &["lisp", "lsp", "nl"],
        media_types: &["text/x-common-lisp"],
        signatures: &[],
        related_formats: &[],
    },
};
