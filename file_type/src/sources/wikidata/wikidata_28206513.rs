use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206513: FileFormat = FileFormat {
    id: 28_206_513,
    puid: "wikidata/28206513",
    name: "LSS16",
    extensions: &["16", "lss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
