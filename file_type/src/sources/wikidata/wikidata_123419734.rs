use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123419734: FileType = FileType {
    file_format: &FileFormat {
        id: 123_419_734,
        source_type: SourceType::Wikidata,
        name: "StuffIt Zip Archive",
        extensions: &["zip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
