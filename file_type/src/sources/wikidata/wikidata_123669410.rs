use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123669410: FileType = FileType {
    file_format: &FileFormat {
        id: 123_669_410,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing X6",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
