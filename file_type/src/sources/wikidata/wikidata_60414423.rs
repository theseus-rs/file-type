use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60414423: FileFormat = FileFormat {
    id: 60_414_423,
    puid: "wikidata/60414423",
    name: "TAP (Commodore 64)",
    extensions: &["tap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
