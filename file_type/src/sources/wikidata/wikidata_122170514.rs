use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122170514: FileFormat = FileFormat {
    id: 122_170_514,
    puid: "wikidata/122170514",
    name: "WireGuard profile",
    extensions: &["conf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
