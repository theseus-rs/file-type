use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851435: FileFormat = FileFormat {
    id: 105_851_435,
    puid: "wikidata/105851435",
    name: "Windows 8-10 Desktop Theme",
    extensions: &["theme"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
