use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_186: FileFormat = FileFormat {
    id: 186,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Workspace File",
    extensions: &["xlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasLowerPriorityThan,
        id: 2_712,
    }],
};
