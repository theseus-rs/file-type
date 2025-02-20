use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5924007: FileType = FileType {
    file_format: &FileFormat {
        id: 5_924_007,
        source_type: SourceType::Wikidata,
        name: "JavaScript format",
        extensions: &["js"],
        media_types: &["text/javascript"],
        signatures: &[],
        related_formats: &[],
    },
};
