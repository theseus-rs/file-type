use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851008: FileFormat = FileFormat {
    id: 105_851_008,
    puid: "wikidata/105851008",
    name: "Windows Desktop Theme (with rem)",
    extensions: &["the", "theme"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
