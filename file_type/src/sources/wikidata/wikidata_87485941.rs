use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87485941: FileFormat = FileFormat {
    id: 87_485_941,
    puid: "wikidata/87485941",
    name: "SketchUp Document 5",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
