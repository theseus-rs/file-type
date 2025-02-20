use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111440875: FileType = FileType {
    file_format: &FileFormat {
        id: 111_440_875,
        source_type: SourceType::Wikidata,
        name: "Tcl Script",
        extensions: &["tcl"],
        media_types: &["text/tcl"],
        signatures: &[],
        related_formats: &[],
    },
};
