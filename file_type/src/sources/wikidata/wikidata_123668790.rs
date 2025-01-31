use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123668790: FileFormat = FileFormat {
    id: 123_668_790,
    puid: "wikidata/123668790",
    name: "CorelDraw Drawing 8 Bidi",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
