use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_219983: FileFormat = FileFormat {
    id: 219_983,
    puid: "wikidata/219983",
    name: "Zoo",
    extensions: &["zoo"],
    media_types: &["application/x-zoo"],
    internal_signatures: &[],
    related_formats: &[],
};
