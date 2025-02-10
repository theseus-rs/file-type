use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52060319: FileType = FileType {
    file_format: &FileFormat {
        id: 52_060_319,
        source_type: SourceType::Wikidata,
        name: "JustWrite Text Document",
        extensions: &["jw", "jwt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
