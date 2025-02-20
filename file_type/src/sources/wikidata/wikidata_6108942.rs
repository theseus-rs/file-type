use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6108942: FileType = FileType {
    file_format: &FileFormat {
        id: 6_108_942,
        source_type: SourceType::Wikidata,
        name: "JSON-LD",
        extensions: &["jsonld"],
        media_types: &["application/ld+json"],
        signatures: &[],
        related_formats: &[],
    },
};
