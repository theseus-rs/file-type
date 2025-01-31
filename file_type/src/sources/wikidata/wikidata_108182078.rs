use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108182078: FileFormat = FileFormat {
    id: 108_182_078,
    puid: "wikidata/108182078",
    name: "Android App Bundle",
    extensions: &["aab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
