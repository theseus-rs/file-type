use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_66: FileFormat = FileFormat {
    id: 66,
    source_type: SourceType::Linguist,
    name: "Common Lisp",
    extensions: &["asd", "cl", "l", "lisp", "lsp", "ny", "podsl", "sexp"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
