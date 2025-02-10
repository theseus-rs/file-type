use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_10846539: FileType = FileType {
    file_format: &FileFormat {
        id: 10_846_539,
        source_type: SourceType::Wikidata,
        name: "BNA",
        extensions: &["bna"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
