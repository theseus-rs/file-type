use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28401268: FileFormat = FileFormat {
    id: 28_401_268,
    puid: "wikidata/28401268",
    name: "XIP",
    extensions: &["xip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
