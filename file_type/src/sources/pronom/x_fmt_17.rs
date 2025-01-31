use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_17: FileFormat = FileFormat {
    id: 44,
    puid: "x-fmt/17",
    name: "Microsoft Excel Template",
    extensions: &["xlt"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 684,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
