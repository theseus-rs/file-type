use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206284: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_284,
        source_type: SourceType::Wikidata,
        name: "IBM KIPS palette",
        extensions: &["kpl", "pal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
