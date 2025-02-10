use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000657: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_657,
        source_type: SourceType::Wikidata,
        name: "Polygon data file",
        extensions: &["poly"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
