use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47202526: FileType = FileType {
    file_format: &FileFormat {
        id: 47_202_526,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Spreadsheet file format version 6",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
