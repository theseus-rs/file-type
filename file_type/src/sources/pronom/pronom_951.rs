use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_951: FileFormat = FileFormat {
    id: 951,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor for Windows",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
