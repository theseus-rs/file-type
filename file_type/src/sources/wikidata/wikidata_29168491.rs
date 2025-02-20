use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29168491: FileType = FileType {
    file_format: &FileFormat {
        id: 29_168_491,
        source_type: SourceType::Wikidata,
        name: "InfluxDB TSM file",
        extensions: &["tsm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
