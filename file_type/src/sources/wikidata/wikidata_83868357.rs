use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83868357: FileType = FileType {
    file_format: &FileFormat {
        id: 83_868_357,
        source_type: SourceType::Wikidata,
        name: "SOSI, version 4",
        extensions: &["sos"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
