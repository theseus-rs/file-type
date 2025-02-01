use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123678686: FileFormat = FileFormat {
    id: 123_678_686,
    puid: "wikidata/123678686",
    name: "CorelDraw Drawing 2017",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
