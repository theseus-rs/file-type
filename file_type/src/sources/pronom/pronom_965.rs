use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_965: FileFormat = FileFormat {
    id: 965,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor for Windows",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
