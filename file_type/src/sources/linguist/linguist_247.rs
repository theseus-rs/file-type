use crate::format::FileFormat;

pub(crate) const LINGUIST_247: FileFormat = FileFormat {
    id: 247,
    puid: "linguist/247",
    name: "NewLisp",
    extensions: &["lisp", "lsp", "nl"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
