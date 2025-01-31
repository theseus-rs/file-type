use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87657455: FileFormat = FileFormat {
    id: 87_657_455,
    puid: "wikidata/87657455",
    name: "SketchUp Document 18",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
