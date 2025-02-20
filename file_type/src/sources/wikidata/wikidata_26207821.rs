use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26207821: FileType = FileType {
    file_format: &FileFormat {
        id: 26_207_821,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Spreadsheet Document, Strict, ISO/IEC 29500:2012",
        extensions: &["xlsx"],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
        signatures: &[],
        related_formats: &[],
    },
};
