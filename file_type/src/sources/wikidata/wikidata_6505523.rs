use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6505523: FileFormat = FileFormat {
    id: 6_505_523,
    puid: "wikidata/6505523",
    name: "Layered Image File Format",
    extensions: &["liff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
