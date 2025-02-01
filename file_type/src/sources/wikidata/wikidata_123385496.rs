use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123385496: FileFormat = FileFormat {
    id: 123_385_496,
    puid: "wikidata/123385496",
    name: "Path library file",
    extensions: &["ptl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
