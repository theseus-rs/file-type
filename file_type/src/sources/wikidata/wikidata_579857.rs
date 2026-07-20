use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_579857: FileType = FileType {
    file_format: &FileFormat {
        id: 579_857,
        source_type: SourceType::Wikidata,
        name: "FFV1",
        extensions: &[],
        media_types: &["video/FFV1"],
        signatures: &[],
        related_formats: &[],
    },
};
