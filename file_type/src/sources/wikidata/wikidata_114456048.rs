use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114456048: FileType = FileType {
    file_format: &FileFormat {
        id: 114_456_048,
        source_type: SourceType::Wikidata,
        name: "Apache Avro Protocol Data",
        extensions: &["avpr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
