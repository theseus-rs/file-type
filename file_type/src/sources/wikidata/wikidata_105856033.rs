use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856033: FileFormat = FileFormat {
    id: 105_856_033,
    puid: "wikidata/105856033",
    name: "SPECCTRA Design",
    extensions: &["dsn"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
