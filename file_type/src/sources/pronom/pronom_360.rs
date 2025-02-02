use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_360: FileFormat = FileFormat {
    id: 360,
    source_type: SourceType::Pronom,
    name: "Microsoft Project",
    extensions: &["mpp"],
    media_types: &["application/vnd.ms-project"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 359,
    }],
};
