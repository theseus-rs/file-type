use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207044: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_044,
        source_type: SourceType::Wikidata,
        name: "PM",
        extensions: &["pm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
