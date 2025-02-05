use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121840788: FileFormat = FileFormat {
    id: 121_840_788,
    source_type: SourceType::Wikidata,
    name: "Common Instrument File 2",
    extensions: &["ci2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
