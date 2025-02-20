use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83794466: FileType = FileType {
    file_format: &FileFormat {
        id: 83_794_466,
        source_type: SourceType::Wikidata,
        name: "FileMaker Pro Database, version 12",
        extensions: &["fmp12"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
