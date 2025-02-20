use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123436632: FileType = FileType {
    file_format: &FileFormat {
        id: 123_436_632,
        source_type: SourceType::Wikidata,
        name: "Construction File",
        extensions: &["cf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
