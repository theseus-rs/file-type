use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_749179: FileType = FileType {
    file_format: &FileFormat {
        id: 749_179,
        source_type: SourceType::Wikidata,
        name: "WS-Policy",
        extensions: &[],
        media_types: &["application/wspolicy+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
