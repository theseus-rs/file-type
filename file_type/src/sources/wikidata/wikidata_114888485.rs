use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114888485: FileFormat = FileFormat {
    id: 114_888_485,
    puid: "wikidata/114888485",
    name: "Scrapbook Factory Deluxe Web Album file",
    extensions: &["swa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
