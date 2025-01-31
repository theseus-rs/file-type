use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_296924: FileFormat = FileFormat {
    id: 296_924,
    puid: "wikidata/296924",
    name: "ART image file format",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
