use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_377: FileFormat = FileFormat {
    id: 377,
    source_type: SourceType::Pronom,
    name: "Microsoft Visio Drawing",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
