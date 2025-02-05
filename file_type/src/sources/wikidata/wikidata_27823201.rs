use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27823201: FileFormat = FileFormat {
    id: 27_823_201,
    source_type: SourceType::Wikidata,
    name: "Binary Terrain, version 1.3",
    extensions: &["bt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
