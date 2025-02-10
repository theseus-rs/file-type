use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61752032: FileType = FileType {
    file_format: &FileFormat {
        id: 61_752_032,
        source_type: SourceType::Wikidata,
        name: "FileMaker Pro Database, version 7",
        extensions: &["fp7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
