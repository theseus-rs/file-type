use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25305144: FileFormat = FileFormat {
    id: 25_305_144,
    puid: "wikidata/25305144",
    name: "ShrinkIt",
    extensions: &["shk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
