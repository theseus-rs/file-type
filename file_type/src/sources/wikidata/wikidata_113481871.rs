use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113481871: FileType = FileType {
    file_format: &FileFormat {
        id: 113_481_871,
        source_type: SourceType::Wikidata,
        name: "Calc602 Spreadsheet file or backup file",
        extensions: &["bak", "tc6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
