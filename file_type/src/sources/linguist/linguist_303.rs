use crate::format::FileFormat;

pub(crate) const LINGUIST_303: FileFormat = FileFormat {
    id: 303,
    puid: "linguist/303",
    name: "Python",
    extensions: &[
        "cgi", "fcgi", "gyp", "gypi", "lmi", "py", "py3", "pyde", "pyi", "pyp", "pyt", "pyw",
        "rpy", "spec", "tac", "wsgi", "xpy",
    ],
    media_types: &["text/x-python"],
    internal_signatures: &[],
    related_formats: &[],
};
