use crate::format::FileFormat;

pub(crate) const LINGUIST_368: FileFormat = FileFormat {
    id: 368,
    puid: "linguist/368",
    name: "Tcsh",
    extensions: &["csh", "tcsh"],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
