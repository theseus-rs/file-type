use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28212272: FileType = FileType {
    file_format: &FileFormat {
        id: 28_212_272,
        source_type: SourceType::Wikidata,
        name: "Noweb",
        extensions: &["nw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
