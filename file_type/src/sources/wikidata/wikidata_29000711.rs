use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000711: FileFormat = FileFormat {
    id: 29_000_711,
    puid: "wikidata/29000711",
    name: "TM",
    extensions: &["tm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
