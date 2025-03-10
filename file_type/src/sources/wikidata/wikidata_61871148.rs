use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61871148: FileType = FileType {
    file_format: &FileFormat {
        id: 61_871_148,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Spreadsheet for Windows, version 3.0a",
        extensions: &[],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
