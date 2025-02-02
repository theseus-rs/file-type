use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_964: FileFormat = FileFormat {
    id: 964,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor for Windows",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
