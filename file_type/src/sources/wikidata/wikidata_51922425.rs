use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51922425: FileType = FileType {
    file_format: &FileFormat {
        id: 51_922_425,
        source_type: SourceType::Wikidata,
        name: "Quicken Data File",
        extensions: &["abd", "qdf", "qel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
