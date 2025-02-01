use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114888746: FileFormat = FileFormat {
    id: 114_888_746,
    puid: "wikidata/114888746",
    name: "Scrapbook Factory Deluxe Envelope file",
    extensions: &["sev"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
