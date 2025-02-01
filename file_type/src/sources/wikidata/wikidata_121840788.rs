use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121840788: FileFormat = FileFormat {
    id: 121_840_788,
    puid: "wikidata/121840788",
    name: "Common Instrument File 2",
    extensions: &["ci2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
