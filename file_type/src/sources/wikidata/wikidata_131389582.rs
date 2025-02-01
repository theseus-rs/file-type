use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131389582: FileFormat = FileFormat {
    id: 131_389_582,
    puid: "wikidata/131389582",
    name: "Velocity file format",
    extensions: &["vm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
