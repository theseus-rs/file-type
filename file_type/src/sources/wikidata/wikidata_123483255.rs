use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123483255: FileType = FileType {
    file_format: &FileFormat {
        id: 123_483_255,
        source_type: SourceType::Wikidata,
        name: "C extension for CPython on Windows (.pyd)",
        extensions: &["pyd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
