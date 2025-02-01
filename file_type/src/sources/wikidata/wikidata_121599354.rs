use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121599354: FileFormat = FileFormat {
    id: 121_599_354,
    puid: "wikidata/121599354",
    name: "Hallmark Card Studio format",
    extensions: &["hcs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
