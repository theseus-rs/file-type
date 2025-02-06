use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130001193: FileFormat = FileFormat {
    id: 130_001_193,
    source_type: SourceType::Wikidata,
    name: "Jsonnet source code file",
    extensions: &["jsonnet"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
