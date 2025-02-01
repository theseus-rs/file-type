use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123385570: FileFormat = FileFormat {
    id: 123_385_570,
    puid: "wikidata/123385570",
    name: "Scene library file",
    extensions: &["scl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
