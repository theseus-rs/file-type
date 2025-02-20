use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128792169: FileType = FileType {
    file_format: &FileFormat {
        id: 128_792_169,
        source_type: SourceType::Wikidata,
        name: "Cypher query format",
        extensions: &["cyp", "cypher"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
