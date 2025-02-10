use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114050529: FileType = FileType {
    file_format: &FileFormat {
        id: 114_050_529,
        source_type: SourceType::Wikidata,
        name: "Canon MIF File",
        extensions: &["mif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
