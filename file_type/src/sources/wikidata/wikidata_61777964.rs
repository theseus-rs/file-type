use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61777964: FileFormat = FileFormat {
    id: 61_777_964,
    puid: "wikidata/61777964",
    name: "Play SID Audio, version 2",
    extensions: &["psid", "sid"],
    media_types: &["audio/prs.sid", "audio/prs.sid"],
    internal_signatures: &[],
    related_formats: &[],
};
