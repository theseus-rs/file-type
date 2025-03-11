use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61766831: FileType = FileType {
    file_format: &FileFormat {
        id: 61_766_831,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Spreadsheet for Windows, version 2",
        extensions: &[],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
