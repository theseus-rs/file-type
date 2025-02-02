use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const CUSTOM_4: FileFormat = FileFormat {
    id: 4,
    source_type: SourceType::Custom,
    name: "JSON Lines",
    extensions: &["jsonl"],
    media_types: &["application/jsonl"],
    internal_signatures: &[],
    related_formats: &[],
};
