use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_84842889: FileFormat = FileFormat {
    id: 84_842_889,
    puid: "wikidata/84842889",
    name: "GL Transmission Format, version 2 (text)",
    extensions: &["gltf"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
