use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967196: FileFormat = FileFormat {
    id: 27_967_196,
    puid: "wikidata/27967196",
    name: "Impulse Tracker sample",
    extensions: &["its"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
