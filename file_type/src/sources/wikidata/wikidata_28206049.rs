use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206049: FileFormat = FileFormat {
    id: 28_206_049,
    source_type: SourceType::Wikidata,
    name: "ERDAS Imagine IMG",
    extensions: &["img"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
