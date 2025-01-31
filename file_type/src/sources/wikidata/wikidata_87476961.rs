use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87476961: FileFormat = FileFormat {
    id: 87_476_961,
    puid: "wikidata/87476961",
    name: "SketchUp Document 2",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
