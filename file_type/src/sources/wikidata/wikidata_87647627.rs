use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87647627: FileFormat = FileFormat {
    id: 87_647_627,
    puid: "wikidata/87647627",
    name: "SketchUp Document 15",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
