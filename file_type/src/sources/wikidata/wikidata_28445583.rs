use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445583: FileFormat = FileFormat {
    id: 28_445_583,
    puid: "wikidata/28445583",
    name: "Application Label Cache",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
