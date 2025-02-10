use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123680176: FileType = FileType {
    file_format: &FileFormat {
        id: 123_680_176,
        source_type: SourceType::Wikidata,
        name: "S-57 Electronic Navigational Chart 3.1",
        extensions: &["000", "001", "002", "003", "004", "005", "006"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
