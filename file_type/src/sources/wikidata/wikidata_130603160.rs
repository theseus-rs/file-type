use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130603160: FileType = FileType {
    file_format: &FileFormat {
        id: 130_603_160,
        source_type: SourceType::Wikidata,
        name: "REBOL file format",
        extensions: &["r", "r3", "reb"],
        media_types: &["text/x-rebol"],
        signatures: &[],
        related_formats: &[],
    },
};
