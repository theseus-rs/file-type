use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87574044: FileFormat = FileFormat {
    id: 87_574_044,
    puid: "wikidata/87574044",
    name: "SketchUp Document 14",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
