use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123678779: FileFormat = FileFormat {
    id: 123_678_779,
    puid: "wikidata/123678779",
    name: "CorelDraw Drawing 2018",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
