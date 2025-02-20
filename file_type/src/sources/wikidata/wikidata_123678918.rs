use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123678918: FileType = FileType {
    file_format: &FileFormat {
        id: 123_678_918,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing 2019",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
