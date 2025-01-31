use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51839189: FileFormat = FileFormat {
    id: 51_839_189,
    puid: "wikidata/51839189",
    name: "Broderbund Print Shop Deluxe Pamphlet",
    extensions: &["pdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
