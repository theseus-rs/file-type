use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123679549: FileFormat = FileFormat {
    id: 123_679_549,
    puid: "wikidata/123679549",
    name: "CorelDraw Drawing 2021",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
