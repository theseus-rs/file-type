use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105854466: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_466,
        source_type: SourceType::Wikidata,
        name: "asciicast (v2)",
        extensions: &["cast"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
