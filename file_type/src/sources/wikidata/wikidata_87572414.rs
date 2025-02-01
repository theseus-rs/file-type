use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87572414: FileFormat = FileFormat {
    id: 87_572_414,
    puid: "wikidata/87572414",
    name: "SketchUp Document 13",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
