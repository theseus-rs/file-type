use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852978: FileFormat = FileFormat {
    id: 105_852_978,
    puid: "wikidata/105852978",
    name: "Session Description Protocol (with rem)",
    extensions: &["sdp"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
