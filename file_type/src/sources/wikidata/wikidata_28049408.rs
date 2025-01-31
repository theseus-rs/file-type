use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049408: FileFormat = FileFormat {
    id: 28_049_408,
    puid: "wikidata/28049408",
    name: "DEGAS image, low resolution",
    extensions: &["PI1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
