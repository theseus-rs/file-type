use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123385601: FileFormat = FileFormat {
    id: 123_385_601,
    source_type: SourceType::Wikidata,
    name: "Sceneeffect library file",
    extensions: &["sel"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
