use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1004: FileFormat = FileFormat {
    id: 1_004,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor DOS",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
