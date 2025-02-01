use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67123981: FileFormat = FileFormat {
    id: 67_123_981,
    puid: "wikidata/67123981",
    name: "Print Artist craft file format",
    extensions: &["crf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
