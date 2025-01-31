use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206232: FileFormat = FileFormat {
    id: 28_206_232,
    puid: "wikidata/28206232",
    name: "HP Paintjet",
    extensions: &["pjx1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
