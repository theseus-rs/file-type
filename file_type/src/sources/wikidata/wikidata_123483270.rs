use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123483270: FileFormat = FileFormat {
    id: 123_483_270,
    puid: "wikidata/123483270",
    name: "executable Python zip archive (.pyz)",
    extensions: &["pyz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
