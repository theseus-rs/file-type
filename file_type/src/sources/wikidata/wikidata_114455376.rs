use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114455376: FileType = FileType {
    file_format: &FileFormat {
        id: 114_455_376,
        source_type: SourceType::Wikidata,
        name: "Apache Avro Schema file format",
        extensions: &["avsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
