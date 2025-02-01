use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123483255: FileFormat = FileFormat {
    id: 123_483_255,
    puid: "wikidata/123483255",
    name: "C extension for CPython on Windows (.pyd)",
    extensions: &["pyd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
