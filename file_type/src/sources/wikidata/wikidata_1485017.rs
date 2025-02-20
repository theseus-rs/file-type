use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1485017: FileType = FileType {
    file_format: &FileFormat {
        id: 1_485_017,
        source_type: SourceType::Wikidata,
        name: "GDSII stream format",
        extensions: &["gds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
