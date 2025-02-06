use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_989: FileFormat = FileFormat {
    id: 989,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor Windows",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
