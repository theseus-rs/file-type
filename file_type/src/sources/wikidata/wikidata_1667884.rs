use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1667884: FileType = FileType {
    file_format: &FileFormat {
        id: 1_667_884,
        source_type: SourceType::Wikidata,
        name: "Internationalization Tag Set",
        extensions: &["its"],
        media_types: &["application/its+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
