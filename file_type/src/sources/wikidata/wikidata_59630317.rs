use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59630317: FileFormat = FileFormat {
    id: 59_630_317,
    puid: "wikidata/59630317",
    name: "P00 C64 Image Format",
    extensions: &["p00", "p01", "p02", "p03", "p04"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
