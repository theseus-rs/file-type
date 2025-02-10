use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60558665: FileType = FileType {
    file_format: &FileFormat {
        id: 60_558_665,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks Spreadsheet, version 2",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
