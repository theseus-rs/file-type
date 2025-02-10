use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_76515023: FileType = FileType {
    file_format: &FileFormat {
        id: 76_515_023,
        source_type: SourceType::Wikidata,
        name: "Safari Web History",
        extensions: &["webhistory"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
