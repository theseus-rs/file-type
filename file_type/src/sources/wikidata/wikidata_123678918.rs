use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123678918: FileFormat = FileFormat {
    id: 123_678_918,
    puid: "wikidata/123678918",
    name: "CorelDraw Drawing 2019",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
