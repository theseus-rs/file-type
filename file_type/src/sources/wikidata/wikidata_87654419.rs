use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87654419: FileFormat = FileFormat {
    id: 87_654_419,
    puid: "wikidata/87654419",
    name: "SketchUp Document 17",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
