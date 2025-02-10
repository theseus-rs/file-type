use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117835509: FileType = FileType {
    file_format: &FileFormat {
        id: 117_835_509,
        source_type: SourceType::Wikidata,
        name: "Generic fax format",
        extensions: &["cg3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
