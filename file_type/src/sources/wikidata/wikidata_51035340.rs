use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51035340: FileType = FileType {
    file_format: &FileFormat {
        id: 51_035_340,
        source_type: SourceType::Wikidata,
        name: "Paradox Database Table, version 5",
        extensions: &["db"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
