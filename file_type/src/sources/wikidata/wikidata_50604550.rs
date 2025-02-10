use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50604550: FileType = FileType {
    file_format: &FileFormat {
        id: 50_604_550,
        source_type: SourceType::Wikidata,
        name: "FileMaker Pro Database, version 2",
        extensions: &["fm"],
        media_types: &["application/x-filemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
