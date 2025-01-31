use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87657661: FileFormat = FileFormat {
    id: 87_657_661,
    puid: "wikidata/87657661",
    name: "SketchUp Document 19",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
