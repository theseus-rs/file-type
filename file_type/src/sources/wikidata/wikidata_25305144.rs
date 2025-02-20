use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
