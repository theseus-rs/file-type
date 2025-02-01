use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111317643: FileFormat = FileFormat {
    id: 111_317_643,
    puid: "wikidata/111317643",
    name: "Miles Sound System compressed DLS",
    extensions: &["mls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
