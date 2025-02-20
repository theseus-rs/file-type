use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47486948: FileType = FileType {
    file_format: &FileFormat {
        id: 47_486_948,
        source_type: SourceType::Wikidata,
        name: "Silo",
        extensions: &["silo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
