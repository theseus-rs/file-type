use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27910000: FileType = FileType {
    file_format: &FileFormat {
        id: 27_910_000,
        source_type: SourceType::Wikidata,
        name: "RADARSAT-2 Product Information File",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
