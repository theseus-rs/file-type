use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63082925: FileFormat = FileFormat {
    id: 63_082_925,
    puid: "wikidata/63082925",
    name: "Office Open XML Spreadsheet Document",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[],
};
