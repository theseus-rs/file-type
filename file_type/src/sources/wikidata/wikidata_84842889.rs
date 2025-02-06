use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84842889: FileFormat = FileFormat {
    id: 84_842_889,
    source_type: SourceType::Wikidata,
    name: "GL Transmission Format, version 2 (text)",
    extensions: &["gltf"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
