use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_986448: FileType = FileType {
    file_format: &FileFormat {
        id: 986_448,
        source_type: SourceType::Wikidata,
        name: "High-Efficiency Advanced Audio Coding",
        extensions: &[],
        media_types: &["audio/aacp"],
        signatures: &[],
        related_formats: &[],
    },
};
