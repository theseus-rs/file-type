use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114877374: FileFormat = FileFormat {
    id: 114_877_374,
    puid: "wikidata/114877374",
    name: "Scrapbook Factory Deluxe Journal file",
    extensions: &["sjd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
