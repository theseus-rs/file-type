use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117814466: FileType = FileType {
    file_format: &FileFormat {
        id: 117_814_466,
        source_type: SourceType::Wikidata,
        name: "AdTech perfectfax",
        extensions: &["adt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
