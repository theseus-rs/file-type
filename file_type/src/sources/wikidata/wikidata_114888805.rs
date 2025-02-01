use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114888805: FileFormat = FileFormat {
    id: 114_888_805,
    puid: "wikidata/114888805",
    name: "Scrapbook Factory Deluxe Paper file",
    extensions: &["sdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
