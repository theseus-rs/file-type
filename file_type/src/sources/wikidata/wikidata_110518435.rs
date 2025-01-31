use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110518435: FileFormat = FileFormat {
    id: 110_518_435,
    puid: "wikidata/110518435",
    name: "ISDOC Information System Document, version 6.0.1",
    extensions: &["isdoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
