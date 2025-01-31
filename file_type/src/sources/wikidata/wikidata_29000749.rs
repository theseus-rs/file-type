use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000749: FileFormat = FileFormat {
    id: 29_000_749,
    puid: "wikidata/29000749",
    name: "YASRT Raytracer Scene",
    extensions: &["yst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
