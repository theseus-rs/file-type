use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28344898: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_898,
        source_type: SourceType::Wikidata,
        name: "Axc",
        extensions: &["axc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
