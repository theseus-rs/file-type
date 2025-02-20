use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_25824045: FileType = FileType {
    file_format: &FileFormat {
        id: 25_824_045,
        source_type: SourceType::Wikidata,
        name: "OSM Note File",
        extensions: &["osn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
