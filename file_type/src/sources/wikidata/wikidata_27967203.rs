use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967203: FileFormat = FileFormat {
    id: 27_967_203,
    source_type: SourceType::Wikidata,
    name: "NoiseTracker module",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
