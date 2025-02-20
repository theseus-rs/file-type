use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118383473: FileType = FileType {
    file_format: &FileFormat {
        id: 118_383_473,
        source_type: SourceType::Wikidata,
        name: "Album Book file",
        extensions: &["opf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
