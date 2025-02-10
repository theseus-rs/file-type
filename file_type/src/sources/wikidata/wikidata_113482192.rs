use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113482192: FileType = FileType {
    file_format: &FileFormat {
        id: 113_482_192,
        source_type: SourceType::Wikidata,
        name: "Calc602 Spreadsheet file or backup file v.1.00",
        extensions: &["bak", "tc6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
