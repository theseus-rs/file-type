use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48915661: FileType = FileType {
    file_format: &FileFormat {
        id: 48_915_661,
        source_type: SourceType::Wikidata,
        name: "Interleaf Document",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
