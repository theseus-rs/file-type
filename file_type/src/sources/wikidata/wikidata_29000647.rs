use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000647: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_647,
        source_type: SourceType::Wikidata,
        name: "PLG",
        extensions: &["plg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
