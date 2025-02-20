use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47486941: FileType = FileType {
    file_format: &FileFormat {
        id: 47_486_941,
        source_type: SourceType::Wikidata,
        name: "Silo",
        extensions: &["silo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
