use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853930: FileFormat = FileFormat {
    id: 105_853_930,
    puid: "wikidata/105853930",
    name: "Atom web feed",
    extensions: &["atom", "xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
