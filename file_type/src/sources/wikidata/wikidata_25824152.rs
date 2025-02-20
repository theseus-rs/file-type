use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_25824152: FileType = FileType {
    file_format: &FileFormat {
        id: 25_824_152,
        source_type: SourceType::Wikidata,
        name: "JOSM Session File",
        extensions: &["jos"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
