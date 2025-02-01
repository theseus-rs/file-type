use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87566099: FileFormat = FileFormat {
    id: 87_566_099,
    puid: "wikidata/87566099",
    name: "SketchUp Document 6",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
