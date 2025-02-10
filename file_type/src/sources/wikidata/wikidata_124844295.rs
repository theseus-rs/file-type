use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124844295: FileType = FileType {
    file_format: &FileFormat {
        id: 124_844_295,
        source_type: SourceType::Wikidata,
        name: "CyberLink MediaShow Data",
        extensions: &["flz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
