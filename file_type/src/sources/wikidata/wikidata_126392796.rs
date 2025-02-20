use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126392796: FileType = FileType {
    file_format: &FileFormat {
        id: 126_392_796,
        source_type: SourceType::Wikidata,
        name: "Fotoman RAW",
        extensions: &["pxn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
