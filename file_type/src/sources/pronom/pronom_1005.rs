use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1005: FileFormat = FileFormat {
    id: 1_005,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor DOS",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
