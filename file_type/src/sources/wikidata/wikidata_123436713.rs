use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123436713: FileFormat = FileFormat {
    id: 123_436_713,
    puid: "wikidata/123436713",
    name: "Single-Molecule Dataset file",
    extensions: &["smd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
