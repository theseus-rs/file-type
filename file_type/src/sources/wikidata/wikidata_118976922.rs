use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118976922: FileType = FileType {
    file_format: &FileFormat {
        id: 118_976_922,
        source_type: SourceType::Wikidata,
        name: "FreeHand Template 11",
        extensions: &["ft11"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
