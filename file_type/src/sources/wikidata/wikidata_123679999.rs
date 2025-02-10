use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123679999: FileType = FileType {
    file_format: &FileFormat {
        id: 123_679_999,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing 2023",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
