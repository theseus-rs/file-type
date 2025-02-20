use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_10397010: FileType = FileType {
    file_format: &FileFormat {
        id: 10_397_010,
        source_type: SourceType::Wikidata,
        name: ".rmp",
        extensions: &["rmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
