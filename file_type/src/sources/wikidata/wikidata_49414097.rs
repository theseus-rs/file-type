use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49414097: FileType = FileType {
    file_format: &FileFormat {
        id: 49_414_097,
        source_type: SourceType::Wikidata,
        name: "CATIA Model, version 4",
        extensions: &["mod", "model"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
