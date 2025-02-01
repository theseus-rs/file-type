use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120731961: FileFormat = FileFormat {
    id: 120_731_961,
    puid: "wikidata/120731961",
    name: "Amapi Pro file",
    extensions: &["a3p"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
