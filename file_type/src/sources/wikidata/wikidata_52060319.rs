use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52060319: FileFormat = FileFormat {
    id: 52_060_319,
    puid: "wikidata/52060319",
    name: "JustWrite Text Document",
    extensions: &["jw", "jwt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
