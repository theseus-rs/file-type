use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207342: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_342,
        source_type: SourceType::Wikidata,
        name: "Synu",
        extensions: &["syn", "synu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
