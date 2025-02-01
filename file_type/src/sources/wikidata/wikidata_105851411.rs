use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851411: FileFormat = FileFormat {
    id: 105_851_411,
    puid: "wikidata/105851411",
    name: "Windows 98-7 Desktop Theme (with rem)",
    extensions: &["the", "theme"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
