use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138478743: FileType = FileType {
    file_format: &FileFormat {
        id: 138_478_743,
        source_type: SourceType::Wikidata,
        name: "brson",
        extensions: &["brson"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
