use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122170423: FileFormat = FileFormat {
    id: 122_170_423,
    puid: "wikidata/122170423",
    name: "AmnezaVPN profile",
    extensions: &["vpn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
