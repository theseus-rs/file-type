use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51918805: FileType = FileType {
    file_format: &FileFormat {
        id: 51_918_805,
        source_type: SourceType::Wikidata,
        name: "XYWrite Document, version 3",
        extensions: &["xy3"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
