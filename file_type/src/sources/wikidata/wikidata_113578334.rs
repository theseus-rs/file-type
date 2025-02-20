use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113578334: FileType = FileType {
    file_format: &FileFormat {
        id: 113_578_334,
        source_type: SourceType::Wikidata,
        name: "Music Maker Arrangement File",
        extensions: &["mmm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
