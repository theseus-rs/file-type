use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000711: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_711,
        source_type: SourceType::Wikidata,
        name: "TM",
        extensions: &["tm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
