use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855712: FileFormat = FileFormat {
    id: 105_855_712,
    source_type: SourceType::Wikidata,
    name: "LightWave 3D exported object",
    extensions: &["obj"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
