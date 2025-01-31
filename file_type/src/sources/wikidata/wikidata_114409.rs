use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114409: FileFormat = FileFormat {
    id: 114_409,
    puid: "wikidata/114409",
    name: "Turtle",
    extensions: &["ttl"],
    media_types: &["text/turtle"],
    internal_signatures: &[],
    related_formats: &[],
};
