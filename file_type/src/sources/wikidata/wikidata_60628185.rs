use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60628185: FileFormat = FileFormat {
    id: 60_628_185,
    puid: "wikidata/60628185",
    name: "Wordperfect Secondary File, version 5",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
