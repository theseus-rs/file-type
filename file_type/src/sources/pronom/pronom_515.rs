use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_515: FileFormat = FileFormat {
    id: 515,
    source_type: SourceType::Pronom,
    name: "PageMaker Document",
    extensions: &["pm3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 516,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_522,
        },
    ],
};
