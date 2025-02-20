use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_8041715: FileType = FileType {
    file_format: &FileFormat {
        id: 8_041_715,
        source_type: SourceType::Wikidata,
        name: "XCOFF",
        extensions: &["o"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
