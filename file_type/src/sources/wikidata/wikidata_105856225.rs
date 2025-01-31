use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856225: FileFormat = FileFormat {
    id: 105_856_225,
    puid: "wikidata/105856225",
    name: "Summation Document Image Information Load File",
    extensions: &["dii"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
