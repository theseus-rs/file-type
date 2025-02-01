use crate::format::FileFormat;

pub(crate) const LINGUIST_102: FileFormat = FileFormat {
    id: 102,
    puid: "linguist/102",
    name: "Emacs Lisp",
    extensions: &["el", "emacs", "emacs.desktop"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
