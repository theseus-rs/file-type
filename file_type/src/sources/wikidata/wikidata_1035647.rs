use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1035647: FileFormat = FileFormat {
    id: 1_035_647,
    puid: "wikidata/1035647",
    name: "Card Verifiable Certificate",
    extensions: &["cv", "cvcert"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
