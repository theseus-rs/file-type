use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26207712: FileFormat = FileFormat {
    id: 26_207_712,
    puid: "wikidata/26207712",
    name: "Office Open XML Spreadsheet Document, Strict, ISO/IEC 29500:2008",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[],
};
