use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_25305144: FileType = FileType {
    file_format: &FileFormat {
        id: 25_305_144,
        source_type: SourceType::Wikidata,
        name: "ShrinkIt",
        extensions: &["shk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
