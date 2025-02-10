use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47167402: FileType = FileType {
    file_format: &FileFormat {
        id: 47_167_402,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks Spreadsheet file format",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
