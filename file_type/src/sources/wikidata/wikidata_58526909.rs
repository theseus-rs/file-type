use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58526909: FileType = FileType {
    file_format: &FileFormat {
        id: 58_526_909,
        source_type: SourceType::Wikidata,
        name: "SuperCalc Spreadsheet, version 1",
        extensions: &["cal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
