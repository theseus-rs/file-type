use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26207821: FileFormat = FileFormat {
    id: 26_207_821,
    puid: "wikidata/26207821",
    name: "Office Open XML Spreadsheet Document, Strict, ISO/IEC 29500:2012",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[],
};
