use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_367: FileFormat = FileFormat {
    id: 367,
    source_type: SourceType::Linguist,
    name: "Tcl",
    extensions: &["adp", "sdc", "tcl", "tcl.in", "tm", "xdc"],
    media_types: &["text/x-tcl"],
    signatures: &[],
    related_formats: &[],
};
