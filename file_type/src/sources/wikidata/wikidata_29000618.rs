use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000618: FileFormat = FileFormat {
    id: 29_000_618,
    puid: "wikidata/29000618",
    name: "Hiew Colour Markers",
    extensions: &["cmarkers"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
