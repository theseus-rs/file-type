use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_128: FileFormat = FileFormat {
    id: 186,
    puid: "x-fmt/128",
    name: "Microsoft Excel Workspace File",
    extensions: &["xlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_712,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
