use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_604279: FileType = FileType {
    file_format: &FileFormat {
        id: 604_279,
        source_type: SourceType::Wikidata,
        name: "Dirac",
        extensions: &["drc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
