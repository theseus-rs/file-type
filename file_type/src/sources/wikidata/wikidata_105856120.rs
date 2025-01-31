use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856120: FileFormat = FileFormat {
    id: 105_856_120,
    puid: "wikidata/105856120",
    name: "Distribution Format Exchange Profile",
    extensions: &["dfxp"],
    media_types: &["application/ttml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
