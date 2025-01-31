use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123669410: FileFormat = FileFormat {
    id: 123_669_410,
    puid: "wikidata/123669410",
    name: "CorelDraw Drawing X6",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
