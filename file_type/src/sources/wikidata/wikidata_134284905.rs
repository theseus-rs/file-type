use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134284905: FileType = FileType {
    file_format: &FileFormat {
        id: 134_284_905,
        source_type: SourceType::Wikidata,
        name: "Clipper library file",
        extensions: &["lib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
