use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67206788: FileFormat = FileFormat {
    id: 67_206_788,
    source_type: SourceType::Wikidata,
    name: "FutureSplash Document",
    extensions: &["spa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
