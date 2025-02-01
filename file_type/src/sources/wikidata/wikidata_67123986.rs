use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67123986: FileFormat = FileFormat {
    id: 67_123_986,
    puid: "wikidata/67123986",
    name: "Print Artist envelope file format",
    extensions: &["env"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
