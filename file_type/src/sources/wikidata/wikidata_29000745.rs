use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000745: FileFormat = FileFormat {
    id: 29_000_745,
    puid: "wikidata/29000745",
    name: "MultiGen Flight",
    extensions: &["flt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
