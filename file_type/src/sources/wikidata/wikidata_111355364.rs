use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111355364: FileType = FileType {
    file_format: &FileFormat {
        id: 111_355_364,
        source_type: SourceType::Wikidata,
        name: "Covox 8-bit audio",
        extensions: &["v8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
