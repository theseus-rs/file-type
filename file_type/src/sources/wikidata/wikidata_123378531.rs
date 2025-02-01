use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123378531: FileFormat = FileFormat {
    id: 123_378_531,
    puid: "wikidata/123378531",
    name: "Curve library",
    extensions: &["cvl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
