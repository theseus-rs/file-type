use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67123931: FileFormat = FileFormat {
    id: 67_123_931,
    puid: "wikidata/67123931",
    name: "Print Artist banner file format",
    extensions: &["ban"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
