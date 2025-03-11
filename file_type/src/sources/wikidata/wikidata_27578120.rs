use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27578120: FileType = FileType {
    file_format: &FileFormat {
        id: 27_578_120,
        source_type: SourceType::Wikidata,
        name: "AppleSingle, version 2",
        extensions: &[],
        media_types: &["application/applefile"],
        signatures: &[],
        related_formats: &[],
    },
};
