use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000621: FileFormat = FileFormat {
    id: 29_000_621,
    puid: "wikidata/29000621",
    name: "WinHex.pos",
    extensions: &["pos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
