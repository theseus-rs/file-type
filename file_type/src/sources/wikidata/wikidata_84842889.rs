use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_84842889: FileFormat = FileFormat {
    id: 84_842_889,
    source_type: SourceType::Wikidata,
    name: "GL Transmission Format, version 2 (text)",
    extensions: &["gltf"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
