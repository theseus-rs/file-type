use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51035227: FileType = FileType {
    file_format: &FileFormat {
        id: 51_035_227,
        source_type: SourceType::Wikidata,
        name: "Paradox Database Table, version 4",
        extensions: &["db"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
