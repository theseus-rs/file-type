use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_367: FileType = FileType {
    file_format: &FileFormat {
        id: 367,
        source_type: SourceType::Linguist,
        name: "Tcl",
        extensions: &["adp", "sdc", "tcl", "tcl.in", "tm", "xdc"],
        media_types: &["text/x-tcl"],
        signatures: &[],
        related_formats: &[],
    },
};
