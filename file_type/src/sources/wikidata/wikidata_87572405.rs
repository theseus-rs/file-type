use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87572405: FileFormat = FileFormat {
    id: 87_572_405,
    puid: "wikidata/87572405",
    name: "SketchUp Document 8",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
