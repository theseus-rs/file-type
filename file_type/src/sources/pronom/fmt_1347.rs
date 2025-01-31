use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1347: FileFormat = FileFormat {
    id: 2_165,
    puid: "fmt/1347",
    name: "Autodesk Revit Project File",
    extensions: &["rvt", "rte", "rft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_166,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_167,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_164,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
