use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000621: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_621,
        source_type: SourceType::Wikidata,
        name: "WinHex.pos",
        extensions: &["pos"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
