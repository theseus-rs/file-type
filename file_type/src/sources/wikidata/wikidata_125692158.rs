use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125692158: FileType = FileType {
    file_format: &FileFormat {
        id: 125_692_158,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Spreadsheet Template",
        extensions: &["ots"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
