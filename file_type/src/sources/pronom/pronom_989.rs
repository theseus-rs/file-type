use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_989: FileFormat = FileFormat {
    id: 989,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor Windows",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
