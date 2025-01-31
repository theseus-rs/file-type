use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_8041715: FileFormat = FileFormat {
    id: 8_041_715,
    puid: "wikidata/8041715",
    name: "XCOFF",
    extensions: &["o"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
