use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856967: FileFormat = FileFormat {
    id: 105_856_967,
    puid: "wikidata/105856967",
    name: "jGRASP Project",
    extensions: &["gpj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
