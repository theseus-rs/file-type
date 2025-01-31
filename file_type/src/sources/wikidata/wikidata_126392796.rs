use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126392796: FileFormat = FileFormat {
    id: 126_392_796,
    puid: "wikidata/126392796",
    name: "Fotoman RAW",
    extensions: &["pxn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
