use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123678779: FileType = FileType {
    file_format: &FileFormat {
        id: 123_678_779,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing 2018",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
