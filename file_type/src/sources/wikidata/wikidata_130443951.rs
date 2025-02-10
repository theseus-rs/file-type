use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130443951: FileType = FileType {
    file_format: &FileFormat {
        id: 130_443_951,
        source_type: SourceType::Wikidata,
        name: "Subsembly JSON",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
