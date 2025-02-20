use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123668790: FileType = FileType {
    file_format: &FileFormat {
        id: 123_668_790,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing 8 Bidi",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
