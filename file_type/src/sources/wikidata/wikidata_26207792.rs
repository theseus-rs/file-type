use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26207792: FileFormat = FileFormat {
    id: 26_207_792,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Spreadsheet Document, Strict, ISO/IEC 29500:2011",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    signatures: &[],
    related_formats: &[],
};
