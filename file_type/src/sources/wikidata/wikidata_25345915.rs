use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25345915: FileFormat = FileFormat {
    id: 25_345_915,
    puid: "wikidata/25345915",
    name: "Scratch Project SB",
    extensions: &["sb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
