use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851201: FileFormat = FileFormat {
    id: 105_851_201,
    puid: "wikidata/105851201",
    name: "Windows 98-7 Desktop Theme",
    extensions: &["the", "theme"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
