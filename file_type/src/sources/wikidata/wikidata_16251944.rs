use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_16251944: FileType = FileType {
    file_format: &FileFormat {
        id: 16_251_944,
        source_type: SourceType::Wikidata,
        name: "TransXChange",
        extensions: &["txc", "xml"],
        media_types: &["application/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
