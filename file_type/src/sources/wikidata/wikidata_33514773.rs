use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_33514773: FileType = FileType {
    file_format: &FileFormat {
        id: 33_514_773,
        source_type: SourceType::Wikidata,
        name: "LAS 1.0",
        extensions: &["las", "laz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
