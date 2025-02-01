use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855190: FileFormat = FileFormat {
    id: 105_855_190,
    puid: "wikidata/105855190",
    name: "Fractal Flame Parameters",
    extensions: &["flame"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
