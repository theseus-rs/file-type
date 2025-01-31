use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_22097440: FileFormat = FileFormat {
    id: 22_097_440,
    puid: "wikidata/22097440",
    name: "IPSW",
    extensions: &["ipsw"],
    media_types: &["application/x-itunes-ipsw"],
    internal_signatures: &[],
    related_formats: &[],
};
