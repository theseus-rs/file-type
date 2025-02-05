use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_893: FileFormat = FileFormat {
    id: 893,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor for DOS",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
