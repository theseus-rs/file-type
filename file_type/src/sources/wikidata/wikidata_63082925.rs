use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63082925: FileType = FileType {
    file_format: &FileFormat {
        id: 63_082_925,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Spreadsheet Document",
        extensions: &["xlsx"],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
        signatures: &[],
        related_formats: &[],
    },
};
