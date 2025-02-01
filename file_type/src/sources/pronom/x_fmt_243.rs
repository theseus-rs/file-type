use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_243: FileFormat = FileFormat {
    id: 359,
    puid: "x-fmt/243",
    name: "Microsoft Project",
    extensions: &["mpp"],
    media_types: &["application/vnd.ms-project"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 360,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
