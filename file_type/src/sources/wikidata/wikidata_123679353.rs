use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123679353: FileFormat = FileFormat {
    id: 123_679_353,
    puid: "wikidata/123679353",
    name: "CorelDraw Drawing 2020",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
