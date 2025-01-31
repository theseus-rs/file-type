use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855115: FileFormat = FileFormat {
    id: 105_855_115,
    puid: "wikidata/105855115",
    name: "Akoma Ntoso document",
    extensions: &["xml"],
    media_types: &["application/akn+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
