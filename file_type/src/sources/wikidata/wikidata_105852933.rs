use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852933: FileFormat = FileFormat {
    id: 105_852_933,
    puid: "wikidata/105852933",
    name: "Crimson Fields level (with rem)",
    extensions: &["src"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
