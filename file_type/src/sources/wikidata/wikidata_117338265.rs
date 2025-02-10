use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117338265: FileType = FileType {
    file_format: &FileFormat {
        id: 117_338_265,
        source_type: SourceType::Wikidata,
        name: "Corel Catalog",
        extensions: &["clc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
