use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27823193: FileFormat = FileFormat {
    id: 27_823_193,
    source_type: SourceType::Wikidata,
    name: "Binary Terrain, version 1.1",
    extensions: &["bt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
