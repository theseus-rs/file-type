use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84842870: FileFormat = FileFormat {
    id: 84_842_870,
    source_type: SourceType::Wikidata,
    name: "GL Transmission Format, version 1 (text)",
    extensions: &["gltf"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
