use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856661: FileFormat = FileFormat {
    id: 105_856_661,
    puid: "wikidata/105856661",
    name: "Windows URL shortcut",
    extensions: &["url"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
