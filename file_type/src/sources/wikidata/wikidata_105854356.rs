use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854356: FileFormat = FileFormat {
    id: 105_854_356,
    puid: "wikidata/105854356",
    name: "X-Plane Airfoils",
    extensions: &["afl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
