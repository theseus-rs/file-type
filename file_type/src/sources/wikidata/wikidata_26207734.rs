use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26207734: FileType = FileType {
    file_format: &FileFormat {
        id: 26_207_734,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Spreadsheet Document, Transitional, ISO/IEC 29500:2011",
        extensions: &["xlsx"],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
        signatures: &[],
        related_formats: &[],
    },
};
