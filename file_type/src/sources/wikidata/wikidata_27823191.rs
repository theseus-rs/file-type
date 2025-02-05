use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27823191: FileFormat = FileFormat {
    id: 27_823_191,
    source_type: SourceType::Wikidata,
    name: "Binary Terrain, version 1.0",
    extensions: &["bt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
