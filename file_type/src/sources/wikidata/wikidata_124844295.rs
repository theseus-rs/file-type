use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124844295: FileFormat = FileFormat {
    id: 124_844_295,
    puid: "wikidata/124844295",
    name: "CyberLink MediaShow Data",
    extensions: &["flz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
