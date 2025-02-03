use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_247: FileFormat = FileFormat {
    id: 247,
    source_type: SourceType::Linguist,
    name: "NewLisp",
    extensions: &["lisp", "lsp", "nl"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
