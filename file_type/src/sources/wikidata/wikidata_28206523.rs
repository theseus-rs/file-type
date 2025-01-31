use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206523: FileFormat = FileFormat {
    id: 28_206_523,
    puid: "wikidata/28206523",
    name: "LuraWave",
    extensions: &["lwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
