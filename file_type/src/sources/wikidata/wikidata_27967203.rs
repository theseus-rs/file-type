use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967203: FileFormat = FileFormat {
    id: 27_967_203,
    source_type: SourceType::Wikidata,
    name: "NoiseTracker module",
    extensions: &["mod"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
