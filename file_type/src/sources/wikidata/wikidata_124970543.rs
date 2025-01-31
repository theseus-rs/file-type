use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124970543: FileFormat = FileFormat {
    id: 124_970_543,
    puid: "wikidata/124970543",
    name: "MIX message data file",
    extensions: &["mix"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
