use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_44: FileFormat = FileFormat {
    id: 44,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Template",
    extensions: &["xlt"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 684,
    }],
};
