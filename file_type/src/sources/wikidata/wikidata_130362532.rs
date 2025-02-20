use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130362532: FileType = FileType {
    file_format: &FileFormat {
        id: 130_362_532,
        source_type: SourceType::Wikidata,
        name: "MuPAD file format",
        extensions: &["mu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
