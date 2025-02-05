use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131695920: FileFormat = FileFormat {
    id: 131_695_920,
    source_type: SourceType::Wikidata,
    name: "Chaco graph partitioning output file",
    extensions: &["coords", "graph"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
