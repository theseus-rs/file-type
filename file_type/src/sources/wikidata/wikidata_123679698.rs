use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123679698: FileType = FileType {
    file_format: &FileFormat {
        id: 123_679_698,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing 2022",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
