use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
