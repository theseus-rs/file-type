use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50809874: FileType = FileType {
    file_format: &FileFormat {
        id: 50_809_874,
        source_type: SourceType::Wikidata,
        name: "FileMaker Pro Database, version 1",
        extensions: &[],
        media_types: &["application/x-filemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
