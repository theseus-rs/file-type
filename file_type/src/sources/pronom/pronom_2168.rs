use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2168: FileFormat = FileFormat {
    id: 2_168,
    source_type: SourceType::Pronom,
    name: "Autodesk Revit Project File",
    extensions: &["rvt", "rte"],
    media_types: &[],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_169,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_164,
        },
    ],
};
