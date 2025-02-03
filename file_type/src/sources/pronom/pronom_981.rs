use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_981: FileFormat = FileFormat {
    id: 981,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor Windows",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
