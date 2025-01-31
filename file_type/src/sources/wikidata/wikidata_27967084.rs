use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967084: FileFormat = FileFormat {
    id: 27_967_084,
    puid: "wikidata/27967084",
    name: "Game Music Creator",
    extensions: &["gmc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
