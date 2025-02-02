use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27823193: FileFormat = FileFormat {
    id: 27_823_193,
    source_type: SourceType::Wikidata,
    name: "Binary Terrain, version 1.1",
    extensions: &["bt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
