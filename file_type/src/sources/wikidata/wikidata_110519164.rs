use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110519164: FileFormat = FileFormat {
    id: 110_519_164,
    puid: "wikidata/110519164",
    name: "ISDOCX Information System Document, version 6.0.1",
    extensions: &["isdocx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
