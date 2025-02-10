use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61912820: FileType = FileType {
    file_format: &FileFormat {
        id: 61_912_820,
        source_type: SourceType::Wikidata,
        name: "ODM",
        extensions: &["odm"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
