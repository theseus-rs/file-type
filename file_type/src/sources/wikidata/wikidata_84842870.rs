use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_84842870: FileFormat = FileFormat {
    id: 84_842_870,
    source_type: SourceType::Wikidata,
    name: "GL Transmission Format, version 1 (text)",
    extensions: &["gltf"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
