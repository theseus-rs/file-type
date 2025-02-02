use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26207734: FileFormat = FileFormat {
    id: 26_207_734,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Spreadsheet Document, Transitional, ISO/IEC 29500:2011",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[],
};
