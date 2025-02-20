use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48815611: FileType = FileType {
    file_format: &FileFormat {
        id: 48_815_611,
        source_type: SourceType::Wikidata,
        name: "Framework Database, version 4",
        extensions: &["fw", "fw4"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
