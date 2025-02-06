use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26205771: FileFormat = FileFormat {
    id: 26_205_771,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Spreadsheet Document, Transitional, ISO/IEC 29500:2008",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    signatures: &[],
    related_formats: &[],
};
