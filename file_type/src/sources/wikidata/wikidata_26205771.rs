use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_26205771: FileType = FileType {
    file_format: &FileFormat {
        id: 26_205_771,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Spreadsheet Document, Transitional, ISO/IEC 29500:2008",
        extensions: &["xlsx"],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
        signatures: &[],
        related_formats: &[],
    },
};
