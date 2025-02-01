use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34747905: FileFormat = FileFormat {
    id: 34_747_905,
    puid: "wikidata/34747905",
    name: "Swift script",
    extensions: &["swift"],
    media_types: &["text/x-swift"],
    internal_signatures: &[],
    related_formats: &[],
};
