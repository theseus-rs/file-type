use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206523: FileFormat = FileFormat {
    id: 28_206_523,
    source_type: SourceType::Wikidata,
    name: "LuraWave",
    extensions: &["lwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
