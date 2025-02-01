use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000651: FileFormat = FileFormat {
    id: 29_000_651,
    puid: "wikidata/29000651",
    name: "WLD",
    extensions: &["wld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
