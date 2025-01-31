use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61777776: FileFormat = FileFormat {
    id: 61_777_776,
    puid: "wikidata/61777776",
    name: "Play SID Audio, version 1",
    extensions: &["dxr", "psid"],
    media_types: &["audio/prs.sid", "audio/prs.sid"],
    internal_signatures: &[],
    related_formats: &[],
};
