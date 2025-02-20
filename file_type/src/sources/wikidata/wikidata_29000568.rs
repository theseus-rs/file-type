use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000568: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_568,
        source_type: SourceType::Wikidata,
        name: "Tacx Fortius",
        extensions: &["caf", "imf", "mrlv", "pgmf", "rlv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
