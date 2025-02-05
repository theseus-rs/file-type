use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_966: FileFormat = FileFormat {
    id: 966,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor for Windows",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
