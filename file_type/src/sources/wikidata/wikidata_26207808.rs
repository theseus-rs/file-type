use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26207808: FileFormat = FileFormat {
    id: 26_207_808,
    puid: "wikidata/26207808",
    name: "Office Open XML Spreadsheet Document, Transitional, ISO/IEC 29500:2012",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[],
};
