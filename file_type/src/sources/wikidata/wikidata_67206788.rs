use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67206788: FileFormat = FileFormat {
    id: 67_206_788,
    source_type: SourceType::Wikidata,
    name: "FutureSplash Document",
    extensions: &["spa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
