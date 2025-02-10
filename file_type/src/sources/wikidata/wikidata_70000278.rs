use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_70000278: FileType = FileType {
    file_format: &FileFormat {
        id: 70_000_278,
        source_type: SourceType::Wikidata,
        name: "FAMILYFILE",
        extensions: &["familyfile"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
