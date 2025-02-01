use crate::format::FileFormat;

pub(crate) const LINGUIST_367: FileFormat = FileFormat {
    id: 367,
    puid: "linguist/367",
    name: "Tcl",
    extensions: &["adp", "sdc", "tcl", "tcl.in", "tm", "xdc"],
    media_types: &["text/x-tcl"],
    internal_signatures: &[],
    related_formats: &[],
};
