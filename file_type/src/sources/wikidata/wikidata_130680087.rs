use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130680087: FileType = FileType {
    file_format: &FileFormat {
        id: 130_680_087,
        source_type: SourceType::Wikidata,
        name: "Anvil",
        extensions: &["mca"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
