use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3570403: FileType = FileType {
    file_format: &FileFormat {
        id: 3_570_403,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Spreadsheet Document, ECMA-376 1st Edition",
        extensions: &["xlsx"],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
        signatures: &[],
        related_formats: &[],
    },
};
