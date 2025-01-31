use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123669609: FileFormat = FileFormat {
    id: 123_669_609,
    puid: "wikidata/123669609",
    name: "CorelDraw Drawing X8",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
