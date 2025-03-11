use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61872816: FileType = FileType {
    file_format: &FileFormat {
        id: 61_872_816,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Spreadsheet for Windows, version 3.0b",
        extensions: &[],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
