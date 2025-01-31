use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87481940: FileFormat = FileFormat {
    id: 87_481_940,
    puid: "wikidata/87481940",
    name: "SketchUp Document 4",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
