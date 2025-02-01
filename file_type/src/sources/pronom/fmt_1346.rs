use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1346: FileFormat = FileFormat {
    id: 2_164,
    puid: "fmt/1346",
    name: "Autodesk Revit File",
    extensions: &["rvt", "rfa", "rte", "rft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_165,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_166,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_167,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_168,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_169,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
