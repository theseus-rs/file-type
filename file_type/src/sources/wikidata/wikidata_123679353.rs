use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123679353: FileType = FileType {
    file_format: &FileFormat {
        id: 123_679_353,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing 2020",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
