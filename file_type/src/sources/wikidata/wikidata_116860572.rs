use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116860572: FileType = FileType {
    file_format: &FileFormat {
        id: 116_860_572,
        source_type: SourceType::Wikidata,
        name: "Stock Tracker Report File",
        extensions: &["srw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
