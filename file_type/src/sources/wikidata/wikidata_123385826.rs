use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123385826: FileFormat = FileFormat {
    id: 123_385_826,
    puid: "wikidata/123385826",
    name: "Object Animation file",
    extensions: &["can"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
