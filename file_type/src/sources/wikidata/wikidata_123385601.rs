use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123385601: FileFormat = FileFormat {
    id: 123_385_601,
    source_type: SourceType::Wikidata,
    name: "Sceneeffect library file",
    extensions: &["sel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
