use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6753724: FileType = FileType {
    file_format: &FileFormat {
        id: 6_753_724,
        source_type: SourceType::Wikidata,
        name: "MapInfo TAB format",
        extensions: &["tab"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
