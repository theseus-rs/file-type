use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2063: FileType = FileType {
    file_format: &FileFormat {
        id: 2_063,
        source_type: SourceType::Wikidata,
        name: "JSON",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
