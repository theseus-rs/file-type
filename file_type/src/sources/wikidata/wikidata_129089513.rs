use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129089513: FileFormat = FileFormat {
    id: 129_089_513,
    puid: "wikidata/129089513",
    name: "Embedded Ragel file",
    extensions: &["rl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
