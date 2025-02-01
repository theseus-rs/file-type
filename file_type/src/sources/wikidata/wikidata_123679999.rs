use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123679999: FileFormat = FileFormat {
    id: 123_679_999,
    puid: "wikidata/123679999",
    name: "CorelDraw Drawing 2023",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
