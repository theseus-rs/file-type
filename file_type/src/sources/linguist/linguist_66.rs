use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_66: FileType = FileType {
    file_format: &FileFormat {
        id: 66,
        source_type: SourceType::Linguist,
        name: "Common Lisp",
        extensions: &["asd", "cl", "l", "lisp", "lsp", "ny", "podsl", "sexp"],
        media_types: &["text/x-common-lisp"],
        signatures: &[],
        related_formats: &[],
    },
};
