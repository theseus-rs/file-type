use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131695920: FileFormat = FileFormat {
    id: 131_695_920,
    puid: "wikidata/131695920",
    name: "Chaco graph partitioning output file",
    extensions: &["coords", "graph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
