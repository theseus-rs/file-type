use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_268201: FileFormat = FileFormat {
    id: 268_201,
    source_type: SourceType::Wikidata,
    name: "Wireless Markup Language",
    extensions: &["wml"],
    media_types: &["text/vnd.wap.wml"],
    internal_signatures: &[],
    related_formats: &[],
};
