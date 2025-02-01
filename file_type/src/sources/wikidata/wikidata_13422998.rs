use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_13422998: FileFormat = FileFormat {
    id: 13_422_998,
    puid: "wikidata/13422998",
    name: "HTTP Archive format",
    extensions: &["har"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
