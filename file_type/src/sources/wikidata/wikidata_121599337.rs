use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121599337: FileFormat = FileFormat {
    id: 121_599_337,
    puid: "wikidata/121599337",
    name: "Hallmark Card Studio Project File",
    extensions: &["hmk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
