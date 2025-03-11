use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27578112: FileType = FileType {
    file_format: &FileFormat {
        id: 27_578_112,
        source_type: SourceType::Wikidata,
        name: "AppleSingle, version 1",
        extensions: &[],
        media_types: &["application/applefile"],
        signatures: &[],
        related_formats: &[],
    },
};
