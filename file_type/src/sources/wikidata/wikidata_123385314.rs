use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123385314: FileFormat = FileFormat {
    id: 123_385_314,
    puid: "wikidata/123385314",
    name: "Old material library file",
    extensions: &["mlb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
