use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47195851: FileType = FileType {
    file_format: &FileFormat {
        id: 47_195_851,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Spreadsheet file format, version 5",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
