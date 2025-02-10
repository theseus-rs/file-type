use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117187116: FileType = FileType {
    file_format: &FileFormat {
        id: 117_187_116,
        source_type: SourceType::Wikidata,
        name: "CD Stomper Template file",
        extensions: &["dsu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
