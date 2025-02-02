use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130001193: FileFormat = FileFormat {
    id: 130_001_193,
    source_type: SourceType::Wikidata,
    name: "Jsonnet source code file",
    extensions: &["jsonnet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
