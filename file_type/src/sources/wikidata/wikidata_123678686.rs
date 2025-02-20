use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123678686: FileType = FileType {
    file_format: &FileFormat {
        id: 123_678_686,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing 2017",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
