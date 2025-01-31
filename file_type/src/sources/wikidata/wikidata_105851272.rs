use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851272: FileFormat = FileFormat {
    id: 105_851_272,
    puid: "wikidata/105851272",
    name: "Windows Desktop Theme",
    extensions: &["the", "theme"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
