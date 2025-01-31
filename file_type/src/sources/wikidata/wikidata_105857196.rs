use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857196: FileFormat = FileFormat {
    id: 105_857_196,
    puid: "wikidata/105857196",
    name: "Hydrogen Pattern",
    extensions: &["h2pattern"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
