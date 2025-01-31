use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967380: FileFormat = FileFormat {
    id: 27_967_380,
    puid: "wikidata/27967380",
    name: "EVT",
    extensions: &["evt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
