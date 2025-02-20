use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28445586: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_586,
        source_type: SourceType::Wikidata,
        name: "Application Label Temporary",
        extensions: &["axc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
