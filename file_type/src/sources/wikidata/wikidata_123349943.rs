use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123349943: FileFormat = FileFormat {
    id: 123_349_943,
    puid: "wikidata/123349943",
    name: "Family Origins Database",
    extensions: &["fow"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
