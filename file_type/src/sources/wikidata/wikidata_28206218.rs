use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206218: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_218,
        source_type: SourceType::Wikidata,
        name: "GRF",
        extensions: &["grf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
