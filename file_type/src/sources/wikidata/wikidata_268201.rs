use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_268201: FileFormat = FileFormat {
    id: 268_201,
    source_type: SourceType::Wikidata,
    name: "Wireless Markup Language",
    extensions: &["wml"],
    media_types: &["text/vnd.wap.wml"],
    signatures: &[],
    related_formats: &[],
};
