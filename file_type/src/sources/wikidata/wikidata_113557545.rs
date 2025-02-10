use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113557545: FileType = FileType {
    file_format: &FileFormat {
        id: 113_557_545,
        source_type: SourceType::Wikidata,
        name: "Gear Image",
        extensions: &["p01"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
