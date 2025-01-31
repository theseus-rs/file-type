use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_351: FileFormat = FileFormat {
    id: 515,
    puid: "x-fmt/351",
    name: "PageMaker Document",
    extensions: &["pm3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 516,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_522,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
