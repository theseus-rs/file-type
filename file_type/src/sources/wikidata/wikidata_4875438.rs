use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4875438: FileType = FileType {
    file_format: &FileFormat {
        id: 4_875_438,
        source_type: SourceType::Wikidata,
        name: "Be-Music Source",
        extensions: &["bms"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
