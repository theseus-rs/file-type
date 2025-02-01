use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26205771: FileFormat = FileFormat {
    id: 26_205_771,
    puid: "wikidata/26205771",
    name: "Office Open XML Spreadsheet Document, Transitional, ISO/IEC 29500:2008",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[],
};
