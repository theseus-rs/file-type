use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123378540: FileFormat = FileFormat {
    id: 123_378_540,
    puid: "wikidata/123378540",
    name: "Light library file",
    extensions: &["lgl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
