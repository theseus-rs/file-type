use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125348106: FileType = FileType {
    file_format: &FileFormat {
        id: 125_348_106,
        source_type: SourceType::Wikidata,
        name: "Regularly Sampled Format",
        extensions: &["rsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
