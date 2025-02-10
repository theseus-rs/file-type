use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62414890: FileType = FileType {
    file_format: &FileFormat {
        id: 62_414_890,
        source_type: SourceType::Wikidata,
        name: "Quattro Pro Spreadsheet, version 7-8",
        extensions: &["wb3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
