use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_992: FileFormat = FileFormat {
    id: 992,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor Windows",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
