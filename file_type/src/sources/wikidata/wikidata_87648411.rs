use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87648411: FileFormat = FileFormat {
    id: 87_648_411,
    puid: "wikidata/87648411",
    name: "SketchUp Document 16",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
