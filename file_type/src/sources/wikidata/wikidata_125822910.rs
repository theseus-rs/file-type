use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125822910: FileType = FileType {
    file_format: &FileFormat {
        id: 125_822_910,
        source_type: SourceType::Wikidata,
        name: "Compiled HTML Help index file",
        extensions: &["chw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
