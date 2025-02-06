use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110518435: FileFormat = FileFormat {
    id: 110_518_435,
    source_type: SourceType::Wikidata,
    name: "ISDOC Information System Document, version 6.0.1",
    extensions: &["isdoc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
