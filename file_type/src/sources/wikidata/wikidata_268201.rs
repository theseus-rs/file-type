use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_268201: FileFormat = FileFormat {
    id: 268_201,
    puid: "wikidata/268201",
    name: "Wireless Markup Language",
    extensions: &["wml"],
    media_types: &["text/vnd.wap.wml"],
    internal_signatures: &[],
    related_formats: &[],
};
