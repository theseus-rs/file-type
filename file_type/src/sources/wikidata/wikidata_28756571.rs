use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28756571: FileFormat = FileFormat {
    id: 28_756_571,
    source_type: SourceType::Wikidata,
    name: "Forth script",
    extensions: &["fth"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
