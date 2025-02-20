use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48815440: FileType = FileType {
    file_format: &FileFormat {
        id: 48_815_440,
        source_type: SourceType::Wikidata,
        name: "Framework Database, version 3",
        extensions: &["fw", "fw3"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
