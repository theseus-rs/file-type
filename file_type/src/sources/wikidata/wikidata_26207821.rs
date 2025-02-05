use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26207821: FileFormat = FileFormat {
    id: 26_207_821,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Spreadsheet Document, Strict, ISO/IEC 29500:2012",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    signatures: &[],
    related_formats: &[],
};
